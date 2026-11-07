import os
import shutil
import subprocess
import sys
import threading
import time
import tomllib
from functools import partial
from http.server import HTTPServer, SimpleHTTPRequestHandler
from pathlib import Path

WARNING = '\033[93m'
FAIL = '\033[91m'
HEADER = '\033[95m'
OKGREEN = '\033[92m'
ENDC = '\033[0m'
TARGETS = [
    "x86_64-pc-windows-gnu",
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "aarch64-linux-android",
    "wasm32-unknown-unknown",
]
EXAMPLES = [
    "ball-pit",
    "counter",
    "readme",
    "radio",
    "video-player",
]

def help():
    print()
    print(f"{HEADER}usage:{ENDC} build.py <command> <flags>")
    print()
    print(f"{HEADER}commands:{ENDC}")
    print(f"    {OKGREEN}clippy{ENDC}    checks for formatting issues")
    print(f"    {OKGREEN}check{ENDC}     checks for warnings and errors")
    print(f"    {OKGREEN}targets{ENDC}   installs targets")
    print(f"    {OKGREEN}book{ENDC}      builds the book")
    print()
    print(f"{HEADER}flags:{ENDC}")
    print(f"{OKGREEN} -h --help{ENDC}    prints help message")
    print()

def targets():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py targets <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN}    --dry{ENDC}     doesn't install any targets")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print()

    installed = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout

    print(f"{HEADER}rustup target list --installed{ENDC}")
    print(installed)

    missing = []

    for target in TARGETS:
        print(f"Checking if {target} is installed...")

        if target in installed:
            print(f"{target} is installed.")
        else:
            print(f"{target} is not installed.")
            missing.append(target)

    if len(missing) == 0:
        print(f"{OKGREEN}targets are installed.{ENDC}")
        return 0

    print(f"{HEADER}Missing targets:{ENDC}")

    for target in missing:
        print(target)

    if sys.argv.count("--dry") >= 1:
        print("Aborting for dry run")
        return 0
        
    for target in missing:
        print(f"{HEADER}rustup target add {target}{ENDC}")

        try:
            subprocess.run(
                ["rustup", "target", "add", target],
                check=True,
                stdout=None,
                stderr=subprocess.PIPE,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            if "dns error" in str(error):
                print(f"{FAIL}Failed to add {target} due to DNS error.{ENDC}")
                print(f"{FAIL}Try checking your internet connection{ENDC}")
            else:
                print(f"{WARNING}Failed to add {target}.{ENDC}")
            
            print(e)
            print(error)
            
            return 1

    print(f"{OKGREEN}All targets are installed.{ENDC}")

    return 0

def clippy():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py clippy <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN}    --dry{ENDC}     doesn't run anything")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print()

    print(f"{HEADER}build.py targets{ENDC}")

    code = targets()

    if code != 0:
        return code

    if sys.argv.count("--dry") >= 1:
        print("Aborting for dry run")
        return 0

    for target in TARGETS:
        try:
            print(f"{HEADER}cargo clippy --all-features --target {target}{ENDC}")
            subprocess.run(
                ["cargo", "clippy", "--all-features", "--target", target, "--", "-D", "warnings"],
                check=True,
                stdout=None,
                stderr=subprocess.PIPE,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            
            print(e)
            print(error)
            
            return 1

    print(f"{OKGREEN}No issues{ENDC}")
    return 0

def check():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py check <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN}    --dry{ENDC}     doesn't run anything")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print()

    print(f"{HEADER}build.py targets{ENDC}")

    code = targets()

    if code != 0:
        return code

    if sys.argv.count("--dry") >= 1:
        print("Aborting for dry run")
        return 0

    for target in TARGETS:
        try:
            print(f"{HEADER}cargo check --all-features --target {target}{ENDC}")
            subprocess.run(
                ["cargo", "check", "--all-features", "--target", target],
                check=True,
                stdout=None,
                stderr=subprocess.PIPE,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            print(e)
            print(error)
            return 1

    print(f"{OKGREEN}No issues{ENDC}")
    return 0

def book():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py book <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN} -v --verbose{ENDC}               enables verbose output")
        print(f"{OKGREEN} -h --help{ENDC}                  prints help message")
        print(f"{OKGREEN} -s --serve{ENDC}                 serves the book locally")
        print()
    
    installed = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout

    print(f"{HEADER}rustup target list --installed{ENDC}")
    print(installed)

    if installed.count("wasm32-unknown-unknown") == 0:
        print("wasm32-unknown-unknown isn't installed")
        print(f"{HEADER}rustup target add wasm32-unknown-unknown{ENDC}")

        try:
            subprocess.run(
                ["rustup", "target", "add", "wasm32-unknown-unknown"],
                check=True,
                stdout=None,
                stderr=subprocess.PIPE,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            if "dns error" in str(error):
                print(f"{FAIL}Failed to add wasm32-unknown-unknown due to DNS error.{ENDC}")
                print(f"{FAIL}Try checking your internet connection{ENDC}")
            else:
                print(f"{WARNING}Failed to add wasm32-unknown-unknown.{ENDC}")
            
            print(e)
            print(error)
            
            return 1
    else:
        print(f"{HEADER}wasm32-unknown-unknown is installed{ENDC}")

    installed = subprocess.run(
        ["cargo", "install", "--list"],
        text=True,
        capture_output=True,
        check=True
    ).stdout

    if "mdbook v" not in installed:
        print(f"{HEADER}mdbook is not installed{ENDC}")
        print(f"{HEADER}cargo install mdbook{ENDC}")
        
        try:
            subprocess.run(
                ["cargo", "install", "mdbook"],
                check=True,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            if "dns error" in str(error):
                print(f"{FAIL}Failed to install mdbook due to DNS error.{ENDC}")
                print(f"{FAIL}Try checking your internet connection{ENDC}")
            else:
                print(f"{WARNING}Failed to install mdbook.{ENDC}")

            print(e)
            print(error)
            
            return 1

    print(f"{HEADER}mdbbook is installed{ENDC}")

    print(f"{HEADER}Getting wasm-bindgen version{ENDC}")

    with open("Cargo.lock", "rb") as f:
        lockfile = tomllib.load(f)

    version = None

    for package in lockfile["package"]:
        if package["name"] == "wasm-bindgen":
            version = package["version"]

    if version is None:
        print("Unable to get wasm-bindgen version")
        return 1
    
    if f"wasm-bindgen-cli v{version}" not in installed:
        print(f"{HEADER}wasm-bindgen-cli v{version} is not installed{ENDC}")

        print(f"{HEADER}cargo install -f wasm-bindgen-cli --version {version}{ENDC}")
        try:
            subprocess.run(
                ["cargo", "install", "-f", "wasm-bindgen-cli", "--version", version],
                check=True,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            if "dns error" in str(error):
                print(f"{FAIL}Failed to install wasm-bindgen-cli due to DNS error.{ENDC}")
                print(f"{FAIL}Try checking your internet connection{ENDC}")
            else:
                print(f"{WARNING}Failed to install wasm-bindgen-cli.{ENDC}")

            print(e)
            print(error)
            
            return 1
            
    print(f"{HEADER}wasm-bindgen-cli is installed{ENDC}")

    def build_book():
        try:
            print(f"{HEADER}mdbook build{ENDC}")
            subprocess.run(
                ["mdbook", "build"],
                check=True,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            print("Failed to build book")
            error = e.stderr or ""
            print(e)
            print(error)
            return 1

    def build_examples():
        examples = []
        examples_dir = os.path.join(os.getcwd(), "examples")
        for f in os.listdir(examples_dir):
            if os.path.isdir(os.path.join(examples_dir, f)):
                examples.append(f)

        print(f"Examples: {examples}")
        
        for example in examples:
            try:
                print(f"{HEADER}cargo build --target wasm32-unknown-unknown -p {example} -r -v{ENDC}")
                subprocess.run(
                    ["cargo", "build", "--target", "wasm32-unknown-unknown", "-p", example, "-r"] + (["-v"] if sys.argv.count("-v") >= 1 or sys.argv.count("--verbose") >= 1 else []),
                    check=True,
                    text=True,
                )
            except subprocess.CalledProcessError as e:
                print(f"Failed to build {example}")
                print(e)
                return 1
            try:
                print(f"{HEADER}wasm-bindgen --target web --out-dir target/book/wasm/{example} target/wasm32-unknown-unknown/release/{example}.wasm{ENDC}")
                subprocess.run(
                    ["wasm-bindgen", "--target", "web", "--out-dir", f"target/book/wasm/{example}", f"target/wasm32-unknown-unknown/release/{example}.wasm", "--no-typescript"],
                    check=True,
                    text=True,
                )
            except subprocess.CalledProcessError as e:
                print(f"Failed to generate wasm bindings for {example}")
                print(e)
                return 1
        return 0
    
    def get_file_state(directory):
        return {
            path: path.stat().st_mtime
            for path in directory.rglob("*")
            if path.is_file()
        }
    
    def find_changes(previous, current):
        changes = []
    
        for path in current:
            if path not in previous:
                changes.append(("New file", path))
            elif current[path] != previous[path]:
                changes.append(("File changed", path))
    
        for path in previous:
            if path not in current:
                changes.append(("File deleted", path))
    
        return changes
    
    def book_changed():
        wasm = Path("target/wasm")
        book_wasm = Path("target/book/wasm")
    
        if book_wasm.exists():
            print(f"{HEADER}Moving book/wasm to target/wasm{ENDC}")
            shutil.move(str(book_wasm), str(wasm))
    
        result = build_book()
    
        if wasm.exists():
            print(f"{HEADER}Moving target/wasm to book/wasm{ENDC}")
            shutil.move(str(wasm), str(book_wasm))
    
        return result
    
    def examples_changed():
        print(f"{HEADER}Removing target/book/wasm{ENDC}")
    
        shutil.rmtree("target/book/wasm", ignore_errors=True)
    
        return build_examples()
    
    def run_server():
        handler = partial(
            SimpleHTTPRequestHandler,
            directory="target/book"
        )
    
        server = HTTPServer(("0.0.0.0", 8000), handler)
    
        print("Serving on http://0.0.0.0:8000")
    
        server.serve_forever()
    
    def watch_files():
        directories = {
            Path("./book"): book_changed,
            Path("./examples"): examples_changed,
            Path("./src"): examples_changed,
        }
    
        book_changed()
        examples_changed()
    
        previous = {
            directory: get_file_state(directory)
            for directory in directories
        }
    
        while True:
            for directory, rebuild in directories.items():
                current = get_file_state(directory)
                changes = find_changes(previous[directory], current)
    
                if changes:
                    for change, path in changes:
                        print(f"{change}: {path}")
    
                    rebuild()
    
                previous[directory] = current
    
            time.sleep(0.5)
    
    
    if sys.argv.count("--serve") >= 1 or sys.argv.count("-s") >= 1:
        try:
            server_thread = threading.Thread(target=run_server, daemon=True)
            watcher_thread = threading.Thread(target=watch_files, daemon=True)
            
            server_thread.start()
            watcher_thread.start()
        
            server_thread.join()
            watcher_thread.join()
        except KeyboardInterrupt:
            print()
            return 0
    else:
        if build_book() == 1: return 1
    
        try:
            shutil.rmtree("target/book/wasm", ignore_errors=True)
        except FileNotFoundError:
            pass
        except OSError as e:
            print("Failed to remove target/book/wasm")
            print(e)
            return 1

        return build_examples()

def main():
    if len(sys.argv) < 2:
        help()
        return 2
    elif sys.argv[1] == "--help" or sys.argv[1] == "-h":
        help()
        return 0
    elif sys.argv[1] == "targets":
        return targets()
    elif sys.argv[1] == "clippy":
        return clippy()
    elif sys.argv[1] == "check":
        return check()
    elif sys.argv[1] == "book":
        return book()
    else:
        help()
        return 2

if __name__ == "__main__":
    sys.exit(main())