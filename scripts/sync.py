#!/usr/bin/env python3
"""
Sync script for LeetCode solutions.

This script:
1. Reads problem slugs from problems.yaml
2. Fetches missing data from LeetCode API and caches it in .data/problems_cache.json
3. Creates solution files that don't exist yet
4. Regenerates README.md from template with up-to-date problem list

Usage:
    python3 scripts/sync.py
"""

import json
import re
import sys
from pathlib import Path

import requests
import yaml
from jinja2 import Environment, FileSystemLoader

PROJECT_ROOT = Path(__file__).parent.parent
PROBLEMS_FILE = PROJECT_ROOT / "problems.yaml"
CACHE_DIR = PROJECT_ROOT / ".data"
CACHE_FILE = CACHE_DIR / "problems_cache.json"
SOLUTIONS_DIR = PROJECT_ROOT / "src" / "solutions"
SOLUTIONS_MOD = PROJECT_ROOT / "src" / "solutions.rs"
README_FILE = PROJECT_ROOT / "README.md"
TEMPLATES_DIR = PROJECT_ROOT / "templates"

LEETCODE_API = "https://leetcode.com/graphql"
LEETCODE_PROBLEM_URL = "https://leetcode.com/problems/{slug}/"

GRAPHQL_QUERY = """
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        questionId
        title
        content
        codeSnippets {
            lang
            code
        }
        exampleTestcases
    }
}
"""


def load_problem_slugs():
    """Load problem slugs from problems.yaml"""
    if not PROBLEMS_FILE.exists():
        print(f"Error: {PROBLEMS_FILE} not found")
        print("Create it with problem slugs, e.g.:")
        print("problems:")
        print("  - two-sum")
        print("  - add-two-numbers")
        sys.exit(1)

    with open(PROBLEMS_FILE) as f:
        data = yaml.safe_load(f)

    problems = data.get("problems", [])
    if not problems:
        print(f"Warning: No problems found in {PROBLEMS_FILE}")
        return []

    return problems


def load_cache():
    """Load cached problem data"""
    if not CACHE_FILE.exists():
        return {}

    with open(CACHE_FILE) as f:
        return json.load(f)


def save_cache(cache):
    """Save problem data to cache"""
    CACHE_DIR.mkdir(exist_ok=True)
    with open(CACHE_FILE, "w") as f:
        json.dump(cache, f, indent=2, ensure_ascii=False)


def get_csrf_token(slug):
    """Get CSRF token from LeetCode problem page"""
    url = LEETCODE_PROBLEM_URL.format(slug=slug)
    try:
        response = requests.get(url, timeout=10)
        for cookie in response.cookies:
            if cookie.name == "csrftoken":
                return cookie.value
    except Exception as e:
        print(f"    Warning: Could not get CSRF token: {e}")
    return None


def fetch_problem_data(slug):
    """Fetch problem data from LeetCode GraphQL API"""
    print(f"  Fetching {slug} from LeetCode API...")

    csrf_token = get_csrf_token(slug)

    headers = {
        "Content-Type": "application/json",
        "Referer": LEETCODE_PROBLEM_URL.format(slug=slug),
    }

    if csrf_token:
        headers["Cookie"] = f"csrftoken={csrf_token}"
        headers["x-csrftoken"] = csrf_token

    payload = {
        "operationName": "questionData",
        "variables": {"titleSlug": slug},
        "query": GRAPHQL_QUERY,
    }

    try:
        response = requests.post(
            LEETCODE_API, json=payload, headers=headers, timeout=10
        )

        if response.status_code != 200:
            print(f"    Error: API returned status {response.status_code}")
            return None

        data = response.json()

        if "errors" in data:
            print(f"    Error: {data['errors']}")
            return None

        question = data.get("data", {}).get("question")
        if not question:
            print("    Error: No question data returned")
            return None

        return question

    except Exception as e:
        print(f"    Error: {e}")
        return None


def parse_html_content(html_content):
    """Extract description and examples from HTML content"""
    # Remove HTML tags
    text = re.sub(r"<[^>]+>", "", html_content)
    text = re.sub(r"\n+", "\n", text)
    text = re.sub(r"&lt;", "<", text)
    text = re.sub(r"&gt;", ">", text)
    text = re.sub(r"&nbsp;", " ", text)
    text = re.sub(r"&quot;", '"', text)
    text = re.sub(r"&amp;", "&", text)

    # Extract description (before examples)
    lines = text.split("\n")
    description_lines = []

    for line in lines:
        line = line.strip()
        if (
            line
            and not line.startswith("Example")
            and not line.startswith("Constraints")
        ):
            description_lines.append(line)
        if line.startswith("Example"):
            break

    description = " ".join(description_lines).strip()

    # Extract examples
    examples = []
    example_pattern = r"Example \d+:.*?Input:\s*([^\n]+).*?Output:\s*([^\n]+)"

    for match in re.finditer(example_pattern, html_content, re.DOTALL):
        input_str = match.group(1).strip()
        output_str = match.group(2).strip()

        # Clean up HTML entities
        for pattern, replacement in [
            (r"<[^>]+>", ""),
            (r"&quot;", '"'),
            (r"&lt;", "<"),
            (r"&gt;", ">"),
            (r"&nbsp;", " "),
            (r"&amp;", "&"),
        ]:
            input_str = re.sub(pattern, replacement, input_str).strip()
            output_str = re.sub(pattern, replacement, output_str).strip()

        if input_str and output_str:
            examples.append({"input": input_str, "output": output_str})

    return description, examples


def get_rust_signature(code_snippets):
    """Extract Rust function signature from code snippets"""
    for snippet in code_snippets:
        if snippet["lang"] == "Rust":
            code = snippet["code"]
            match = re.search(r"(pub fn [^{]+)", code)
            if match:
                return match.group(1).strip()
    return None


def process_problem_data(slug, raw_data):
    """Process raw API data into cached format"""
    description, examples = parse_html_content(raw_data["content"])
    rust_signature = get_rust_signature(raw_data["codeSnippets"])

    return {
        "slug": slug,
        "number": int(raw_data["questionId"]),
        "title": raw_data["title"],
        "description": description,
        "signature": rust_signature,
        "examples": examples,
        "url": LEETCODE_PROBLEM_URL.format(slug=slug),
    }


def sync_cache(slugs, cache):
    """Ensure all problem slugs are in cache"""
    updated = False

    for slug in slugs:
        if slug not in cache:
            print(f"  {slug} not in cache, fetching...")
            raw_data = fetch_problem_data(slug)

            if raw_data:
                cache[slug] = process_problem_data(slug, raw_data)
                updated = True
                print(f"    ✓ Cached {slug}")
            else:
                print(f"    ✗ Failed to fetch {slug}")

    return updated


def format_number(num):
    """Format problem number as s0001"""
    return f"s{num:04d}"


def create_solution_file(problem):
    """Create solution file if it doesn't exist"""
    number = problem["number"]
    name = format_number(number)
    filepath = SOLUTIONS_DIR / f"{name}.rs"

    if filepath.exists():
        return False

    # Setup Jinja2
    env = Environment(loader=FileSystemLoader(TEMPLATES_DIR))
    template = env.get_template("solution.rs.j2")

    # Render template with problem data
    content = template.render(
        number=number,
        title=problem["title"],
        description=problem["description"],
        signature=problem.get("signature"),
        examples=problem.get("examples"),
    )

    filepath.write_text(content, encoding="utf-8")
    print(f"    ✓ Created {name}.rs")
    return True


def regenerate_solutions_mod(problems):
    """Regenerate solutions.rs with mod declarations"""
    lines = []
    for problem in sorted(problems, key=lambda x: x["number"]):
        name = format_number(problem["number"])
        lines.append(f"mod {name};")

    content = "\n".join(lines) + "\n"
    SOLUTIONS_MOD.write_text(content, encoding="utf-8")


def regenerate_readme(problems):
    """Regenerate README.md from template"""
    sorted_problems = sorted(problems, key=lambda x: x["number"])

    env = Environment(loader=FileSystemLoader(TEMPLATES_DIR))
    template = env.get_template("README.md.j2")

    content = template.render(
        problems=[
            {
                "title": p["title"],
                "file": f"{format_number(p['number'])}.rs",
                "url": p["url"],
            }
            for p in sorted_problems
        ]
    )

    README_FILE.write_text(content, encoding="utf-8")


def main():
    print("=" * 60)
    print("LeetCode Solutions Sync")
    print("=" * 60)

    # Load problem slugs
    print("\n[1/5] Loading problem slugs...")
    slugs = load_problem_slugs()
    print(f"  Found {len(slugs)} problem(s) in {PROBLEMS_FILE}")

    if not slugs:
        print("\n✓ Nothing to sync")
        return

    # Load and sync cache
    print("\n[2/5] Syncing cache...")
    cache = load_cache()
    cache_updated = sync_cache(slugs, cache)

    if cache_updated:
        save_cache(cache)
        print(f"  ✓ Cache saved to {CACHE_FILE}")
    else:
        print("  ✓ Cache is up to date")

    # Get problems from cache (only those that were successfully fetched)
    problems = [cache[slug] for slug in slugs if slug in cache]

    if not problems:
        print("\n✗ No problems in cache, cannot continue")
        sys.exit(1)

    # Create solution files
    print("\n[3/5] Creating solution files...")
    SOLUTIONS_DIR.mkdir(parents=True, exist_ok=True)
    created = 0
    for problem in problems:
        if create_solution_file(problem):
            created += 1

    if created > 0:
        print(f"  ✓ Created {created} new solution file(s)")
    else:
        print("  ✓ All solution files exist")

    # Regenerate solutions.rs
    print("\n[4/5] Regenerating solutions.rs...")
    regenerate_solutions_mod(problems)
    print(f"  ✓ Updated {SOLUTIONS_MOD}")

    # Regenerate README
    print("\n[5/5] Regenerating README.md...")
    regenerate_readme(problems)
    print(f"  ✓ Updated {README_FILE}")

    print("\n" + "=" * 60)
    print("✓ Sync complete!")
    print("=" * 60)


if __name__ == "__main__":
    main()
