#!/usr/bin/env python3

import shutil
import sys
import tempfile
from pathlib import Path

MOD_RS = "mod.rs"
INPUT_TXT = "input.txt"
YEARS_DIR = "src/years"

YEAR_MOD_TEMPLATE = """use crate::utils::{{file_reader::StdFileReader, solver::Solver}};

{mod_declarations}

pub fn run_day(day: u32) {{
    let reader = StdFileReader;
    match day {{
{match_cases}
        _ => println!("Day {{}} not implemented", day),
    }}
}}
"""

YEARS_MOD_TEMPLATE = """{mod_declarations}

pub fn run_day(year: u32, day: u32) {{
    match year {{
{match_cases}
        _ => println!("Year {{}} not implemented", year),
    }}
}}
"""

MATCH_CASE_TEMPLATE = """        {day} => {{
            let solver =
                day{day_padded}::Day{day_padded}::from_default_path(&reader, {year}, day).expect("Failed to load input");
            solver.solve();
        }}"""

DAY_MOD_TEMPLATE = """use crate::utils::{{file_reader::FileReader, solver::Solver}};

pub struct Day{day_padded} {{
    data: String,
}}

impl Solver for Day{day_padded} {{
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {{
        let data = reader.read_file(file_path)?;
        Ok(Day{day_padded} {{ data }})
    }}

    fn part_one_solution(&self) -> u32 {{
        let is_empty = self.data.is_empty();
        if is_empty { 0 } else { 1 }
    }}

    fn part_two_solution(&self) -> u32 {{
        0
    }}
}}
"""


def scan_years(years_dir: Path) -> list[int]:
    """Scan years directory and return sorted list of year numbers."""
    years = []
    if not years_dir.exists():
        return years

    for item in years_dir.iterdir():
        if item.is_dir() and item.name.startswith("year") and item.name[4:].isdigit():
            years.append(int(item.name[4:]))

    return sorted(years)


def scan_days(year_dir: Path) -> list[int]:
    """Scan year directory and return sorted list of day numbers."""
    days = []
    if not year_dir.exists():
        return days

    for item in year_dir.iterdir():
        if item.is_dir() and item.name.startswith("day") and item.name[3:].isdigit():
            days.append(int(item.name[3:]))

    return sorted(days)


def generate_year_mod_content(year: int, days: list[int]) -> str:
    """Generate year mod.rs content from scratch."""
    mod_declarations = "\n".join(f"pub mod day{day:02d};" for day in days)

    match_cases = "\n".join(
        MATCH_CASE_TEMPLATE.format(day=day, day_padded=f"{day:02d}", year=year)
        for day in days
    )

    return YEAR_MOD_TEMPLATE.format(
        mod_declarations=mod_declarations,
        match_cases=match_cases,
    )


def generate_years_mod_content(years: list[int]) -> str:
    """Generate years mod.rs content from scratch."""
    mod_declarations = "\n".join(f"pub mod year{year};" for year in years)

    match_cases = "\n".join(
        f"        {year} => year{year}::run_day(day)," for year in years
    )

    return YEARS_MOD_TEMPLATE.format(
        mod_declarations=mod_declarations,
        match_cases=match_cases,
    )


def generate_day_mod_content(day_padded: str) -> str:
    """Generate day mod.rs content."""
    return DAY_MOD_TEMPLATE.format(day_padded=day_padded)


def build_in_temp(
    temp_dir: Path,
    base_dir: Path,
    year: int,
    day: int,
    day_padded: str,
) -> dict[Path, Path]:
    """Build all files in temp directory and return mapping of target -> source."""
    years_dir = base_dir / YEARS_DIR
    year_dir = years_dir / f"year{year}"
    day_dir = year_dir / f"day{day_padded}"
    years_mod_file = years_dir / MOD_RS

    day_exists = day_dir.exists() and (day_dir / MOD_RS).exists()

    if day_exists:
        print(
            f"Day {day} for year {year} already exists. Skipping day file creation.",
            file=sys.stderr,
        )

    temp_year_dir = temp_dir / "year"
    temp_day_dir = temp_year_dir / f"day{day_padded}"
    temp_years_mod = temp_dir / "years_mod.rs"

    temp_year_dir.mkdir(parents=True, exist_ok=True)

    existing_days = scan_days(year_dir)
    if day not in existing_days:
        existing_days.append(day)
    existing_days.sort()

    year_mod_content = generate_year_mod_content(year, existing_days)
    (temp_year_dir / MOD_RS).write_text(year_mod_content)

    file_mappings = {
        year_dir / MOD_RS: temp_year_dir / MOD_RS,
    }

    if not day_exists:
        temp_day_dir.mkdir(parents=True, exist_ok=True)
        day_mod_content = generate_day_mod_content(day_padded)
        (temp_day_dir / MOD_RS).write_text(day_mod_content)
        (temp_day_dir / INPUT_TXT).touch()
        file_mappings[day_dir / MOD_RS] = temp_day_dir / MOD_RS
        file_mappings[day_dir / INPUT_TXT] = temp_day_dir / INPUT_TXT

    existing_years = scan_years(years_dir)
    if year not in existing_years:
        existing_years.append(year)
        existing_years.sort()

    years_mod_content = generate_years_mod_content(existing_years)
    temp_years_mod.write_text(years_mod_content)
    file_mappings[years_mod_file] = temp_years_mod

    return file_mappings


def apply_changes(file_mappings: dict[Path, Path]) -> None:
    """Copy files from temp to final locations."""
    for target, source in file_mappings.items():
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, target)


def validate_input(year: str, day: str) -> tuple[int, int]:
    """Validate and convert year and day inputs."""
    if not year.isdigit() or len(year) != 4:
        print("Error: Year must be 4 digits (e.g., 2025)", file=sys.stderr)
        sys.exit(1)

    if not day.isdigit() or not (1 <= int(day) <= 25):
        print("Error: Day must be a number between 1 and 25", file=sys.stderr)
        sys.exit(1)

    return int(year), int(day)


def main() -> None:
    if len(sys.argv) != 3:
        print("Usage: python3 new_day.py <year> <day>", file=sys.stderr)
        print("Example: python3 new_day.py 2025 1", file=sys.stderr)
        sys.exit(1)

    year_str, day_str = sys.argv[1], sys.argv[2]
    year, day = validate_input(year_str, day_str)
    day_padded = f"{day:02d}"

    base_dir = Path(__file__).parent.parent

    temp_dir = Path(tempfile.mkdtemp(prefix=f"aoc_new_day_{year}_{day_padded}_"))
    try:
        file_mappings = build_in_temp(temp_dir, base_dir, year, day, day_padded)
        apply_changes(file_mappings)
        shutil.rmtree(temp_dir)
        print("Success")
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        print(f"Temporary files preserved in: {temp_dir}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
