import os
import shutil
import subprocess
import sys
import threading
import time
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
]

def help():
    print()
    print(f"{HEADER}usage:{ENDC} build.py <command> <flags>")
    print()
    print(f"{HEADER}commands:{ENDC}")
    print(f"    {OKGREEN}build{ENDC}     builds a project")
    print(f"    {OKGREEN}test{ENDC}      runs test suite")
    print(f"    {OKGREEN}fmt{ENDC}       formats code")
    print(f"    {OKGREEN}clippy{ENDC}    checks for formatting issues")
    print(f"    {OKGREEN}check{ENDC}     checks for warnings and errors")
    print(f"    {OKGREEN}targets{ENDC}   installs targets")
    print(f"    {OKGREEN}book{ENDC}      builds the book")
    print(f"    {OKGREEN}doc{ENDC}       builds the documentation")
    print()
    print(f"{HEADER}flags:{ENDC}")
    print(f"{OKGREEN} -v --verbose{ENDC} enables verbose output")
    print(f"{OKGREEN} -h --help{ENDC}    prints help message")
    print()

def targets(user_called):
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py targets <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN} -v --verbose{ENDC} enables verbose output")
        print(f"{OKGREEN}    --dry{ENDC}     doesn't install any targets")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print()

    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1
    installed = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout

    if verbose:
        print(f"{HEADER}rustup target list --installed{ENDC}")
        print(installed)

    missing = []

    for target in TARGETS:
        if verbose:
            print(f"Checking if {target} is installed...")

        if target in installed:
            if verbose:
                print(f"{target} is installed.")
        else:
            if verbose:
                print(f"{target} is not installed.")
            missing.append(target)

    if len(missing) == 0:
        if user_called:
            print(f"{OKGREEN}targets are installed.{ENDC}")
        return 0

    if user_called:
        print(f"{HEADER}Missing targets:{ENDC}")
    
        for target in missing:
            print(target)

    if sys.argv.count("--dry") >= 1:
        if verbose:
            print("Aborting for dry run")
        return 0
        
    for target in missing:
        if verbose:
            print(f"{HEADER}rustup target add {target}{ENDC}")

        try:
            subprocess.run(
                ["rustup", "target", "add", target],
                check=True,
                stdout=None if verbose else subprocess.DEVNULL,
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
            if verbose:
                print(e)
                print(error)
            return 1

    print(f"{OKGREEN}All targets are installed.{ENDC}")

    return 0

def fmt():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py fmt <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN} -v --verbose{ENDC} enables verbose output")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print()
    
    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1
    if verbose:
        print(f"{HEADER}cargo fmt -v{ENDC}")
        subprocess.run(
            ["cargo", "fmt", "-v"],
            check=True,
            text=True,
        )
    else:
        subprocess.run(
            ["cargo", "fmt"],
            check=True,
            text=True,
        )
    return 0

def clippy():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py clippy <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN} -v --verbose{ENDC} enables verbose output")
        print(f"{OKGREEN}    --dry{ENDC}     doesn't run anything")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print()

    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1

    if verbose:
        print(f"{HEADER}build.py targets{ENDC}")

    code = targets(True)

    if code != 0:
        return code

    if sys.argv.count("--dry") >= 1:
        if verbose:
            print("Aborting for dry run")
        return 0

    for target in TARGETS:
        try:
            if verbose:
                print(f"{HEADER}cargo clippy --all-features --target {target}{ENDC}")
            subprocess.run(
                ["cargo", "clippy", "--all-features", "--target", target, "--", "-D", "warnings"],
                check=True,
                stdout=None if verbose else subprocess.DEVNULL,
                stderr=subprocess.PIPE,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            if verbose:
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
        print(f"{OKGREEN} -v --verbose{ENDC} enables verbose output")
        print(f"{OKGREEN}    --dry{ENDC}     doesn't run anything")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print()

    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1

    if verbose:
        print(f"{HEADER}build.py targets{ENDC}")

    code = targets(True)

    if code != 0:
        return code

    if sys.argv.count("--dry") >= 1:
        if verbose:
            print("Aborting for dry run")
        return 0

    for target in TARGETS:
        try:
            if verbose:
                print(f"{HEADER}cargo check --all-features --target {target}{ENDC}")
            subprocess.run(
                ["cargo", "check", "--all-features", "--target", target],
                check=True,
                stdout=None if verbose else subprocess.DEVNULL,
                stderr=subprocess.PIPE,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            error = e.stderr or ""
            if verbose:
                print(e)
                print(error)
            return 1

    print(f"{OKGREEN}No issues{ENDC}")
    return 0

def build():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py build <package> <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN} -v --verbose{ENDC}               enables verbose output")
        print(f"{OKGREEN} -h --help{ENDC}                  prints help message")
        print(f"{OKGREEN} -r --release{ENDC}               builds in release mode")
        print(f"{OKGREEN} -f --features [FEATURE]{ENDC}    builds with the features seperated with a comma or space")
        print(f"{OKGREEN}    --all-features{ENDC}          builds with all features")
        print(f"{OKGREEN} -j --jobs [NUMBER OF JOBS]{ENDC} amount of parallel jobs.")
        print(f"{OKGREEN}    --target [TARGET]{ENDC}       builds for the specified target")
        print()

    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1
    
    command = sys.argv[2:]
    command = ["-F" if arg == "-f" else arg for arg in command]

    if verbose:
        print(command)

    command.insert(0, "cargo")
    command.insert(1, "build")

    try:
        if verbose:
            print(f"{HEADER}{command}{ENDC}")
        subprocess.run(
            command,
            check=True,
            text=True,
        )
    except subprocess.CalledProcessError as e:
        error = e.stderr or ""
        if verbose:
            print(e)
            print(error)
        return 1

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

    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1
    
    installed = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout

    if verbose:
        print(f"{HEADER}rustup target list --installed{ENDC}")
        print(installed)

    if installed.count("wasm32-unknown-unknown") == 0:
        print("wasm32-unknown-unknown isn't installed")

        if verbose:
            print(f"{HEADER}rustup target add wasm32-unknown-unknown{ENDC}")

        try:
            subprocess.run(
                ["rustup", "target", "add", "wasm32-unknown-unknown"],
                check=True,
                stdout=None if verbose else subprocess.DEVNULL,
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
            if verbose:
                print(e)
                print(error)
            return 1
    else:
        if verbose:
            print(f"{HEADER}wasm32-unknown-unknown is installed{ENDC}")

    installed = subprocess.run(
        ["cargo", "install", "--list"],
        text=True,
        capture_output=True,
        check=True
    ).stdout

    if "mdbook v" not in installed:
        print(f"{HEADER}mdbook is not installed{ENDC}")
        if verbose:
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
            if verbose:
                print(e)
                print(error)
            return 1
    if "wasm-bindgen-cli v" not in installed:
        print(f"{HEADER}wasm-bindgen-cli is not installed{ENDC}")
        if verbose:
            print(f"{HEADER}cargo install wasm-bindgen-cli{ENDC}")
        try:
            subprocess.run(
                ["cargo", "install", "wasm-bindgen-cli"],
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
            if verbose:
                print(e)
                print(error)
            return 1

    if sys.argv.count("--serve") >= 1 or sys.argv.count("-s") >= 1:
        def run_server():
            handler = partial(
                SimpleHTTPRequestHandler,
                directory="target/book"
            )
        
            server = HTTPServer(("localhost", 8000), handler)
        
            print("Serving on http://localhost:8000")
        
            server.serve_forever()

        def watch_files():
            def book_changed():
                if verbose:
                    print(f"{HEADER}moving target/book/wasm to target/wasm{ENDC}")
                shutil.move("target/book/wasm", "target/wasm")
                if verbose:
                    print(f"{HEADER}mdbook build{ENDC}")
                subprocess.run(
                    ["mdbook", "build"],
                    check=True,
                    text=True,
                )
                if verbose:
                    print(f"{HEADER}moving target/wasm to target/book/wasm{ENDC}")
                shutil.move("target/wasm", "target/book/wasm")

            def examples_changed():
                if verbose:
                    print(f"{HEADER}removing target/book/wasm{ENDC}")
                try:
                    shutil.rmtree("target/book/wasm", ignore_errors=True)
                except FileNotFoundError:
                    pass
                except OSError as e:
                    print("Failed to remove target/book/wasm")
                    if verbose:
                        print(e)
                    return 1
            
                for example in EXAMPLES:
                    try:
                        if verbose:
                            print(f"{HEADER}cargo build --target wasm32-unknown-unknown -p {example} -r -v{ENDC}")
                        subprocess.run(
                            ["cargo", "build", "--target", "wasm32-unknown-unknown", "-p", example, "-r"] + (["-v"] if verbose else []),
                            check=True,
                            text=True,
                        )
                    except subprocess.CalledProcessError as e:
                        print(f"Failed to build {example}")
                        error = e.stderr or ""
                        if verbose:
                            print(e)
                        return 1
                    try:
                        if verbose:
                            print(f"{HEADER}wasm-bindgen --target web --out-dir target/book/wasm/{example} target/wasm32-unknown-unknown/release/{example}.wasm{ENDC}")
                        subprocess.run(
                            ["wasm-bindgen", "--target", "web", "--out-dir", f"target/book/wasm/{example}", f"target/wasm32-unknown-unknown/release/{example}.wasm", "--no-typescript"],
                            check=True,
                            text=True,
                        )
                    except subprocess.CalledProcessError as e:
                        print(f"Failed to generate wasm bindings for {example}")
                        error = e.stderr or ""
                        if verbose:
                            print(e)
                            print(error)
                        return 1
            
            book = Path("./book")
            examples = Path("./examples")
            src = Path("./src")
        
            previous_book = {
                path: path.stat().st_mtime
                for path in book.rglob("*")
                if path.is_file()
            }
            previous_example = {
                path: path.stat().st_mtime
                for path in examples.rglob("*")
                if path.is_file()
            }
            previous_src = {
                path: path.stat().st_mtime
                for path in src.rglob("*")
                if path.is_file()
            }
        
            while True:
                current_book = {
                    path: path.stat().st_mtime
                    for path in book.rglob("*")
                    if path.is_file()
                }
        
                for path, mtime in current_book.items():
                    if path not in previous_book:
                        if verbose:
                            print(f"New file: {path}")
                        book_changed()
                    elif mtime != previous_book[path]:
                        if verbose:
                            print(f"File changed: {path}")
                        book_changed()
        
                for path in previous_book:
                    if path not in current_book:
                        if verbose:
                            print(f"File deleted: {path}")
                        book_changed()
        
                previous_book = current_book

                current_example = {
                    path: path.stat().st_mtime
                    for path in examples.rglob("*")
                    if path.is_file()
                }

                for path, mtime in current_example.items():
                    if path not in previous_example:
                        if verbose:
                            print(f"New file: {path}")
                        examples_changed()
                    elif mtime != previous_example[path]:
                        if verbose:
                            print(f"File changed: {path}")
                        examples_changed()

                for path in previous_example:
                    if path not in current_example:
                        if verbose:
                            print(f"File deleted: {path}")
                        examples_changed()

                previous_example = current_example

                current_src = {
                    path: path.stat().st_mtime
                    for path in src.rglob("*")
                    if path.is_file()
                }

                for path, mtime in current_src.items():
                    if path not in previous_src:
                        if verbose:
                            print(f"New file: {path}")
                        examples_changed()
                    elif mtime != previous_src[path]:
                        if verbose:
                            print(f"File changed: {path}")
                        examples_changed()

                for path in previous_src:
                    if path not in current_src:
                        if verbose:
                            print(f"File deleted: {path}")
                        examples_changed()

                previous_src = current_src

                time.sleep(0.5)

        server_thread = threading.Thread(target=run_server)
        watcher_thread = threading.Thread(target=watch_files)
        
        server_thread.start()
        watcher_thread.start()
        
        server_thread.join()
        watcher_thread.join()
    else:
        try:
            if verbose:
                print(f"{HEADER}mdbook build{ENDC}")
            subprocess.run(
                ["mdbook", "build"],
                check=True,
                text=True,
            )
        except subprocess.CalledProcessError as e:
            print("Failed to build book")
            error = e.stderr or ""
            if verbose:
                print(e)
                print(error)
            return 1
    
        try:
            shutil.rmtree("target/book/wasm", ignore_errors=True)
        except FileNotFoundError:
            pass
        except OSError as e:
            print("Failed to remove target/book/wasm")
            if verbose:
                print(e)
            return 1
    
        for example in EXAMPLES:
            try:
                if verbose:
                    print(f"{HEADER}cargo build --target wasm32-unknown-unknown -p {example} -r -v{ENDC}")
                subprocess.run(
                    ["cargo", "build", "--target", "wasm32-unknown-unknown", "-p", example, "-r"] + (["-v"] if verbose else []),
                    check=True,
                    text=True,
                )
            except subprocess.CalledProcessError as e:
                print(f"Failed to build {example}")
                error = e.stderr or ""
                if verbose:
                    print(e)
                return 1
            try:
                if verbose:
                    print(f"{HEADER}wasm-bindgen --target web --out-dir target/book/wasm/{example} target/wasm32-unknown-unknown/release/{example}.wasm{ENDC}")
                subprocess.run(
                    ["wasm-bindgen", "--target", "web", "--out-dir", f"target/book/wasm/{example}", f"target/wasm32-unknown-unknown/release/{example}.wasm", "--no-typescript"],
                    check=True,
                    text=True,
                )
            except subprocess.CalledProcessError as e:
                print(f"Failed to generate wasm bindings for {example}")
                error = e.stderr or ""
                if verbose:
                    print(e)
                return 1
        return 0

def test():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py test <testname> <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN} -v --verbose{ENDC} enables verbose output")
        print(f"{OKGREEN} -h --help{ENDC}    prints help message")
        print(f"{OKGREEN} -r --release{ENDC}               builds in release mode")
        print(f"{OKGREEN} -f --features [FEATURE]{ENDC}    builds with the features seperated with a comma or space")
        print(f"{OKGREEN}    --all-features{ENDC}          builds with all features")
        print(f"{OKGREEN} -j --jobs [NUMBER OF JOBS]{ENDC} amount of parallel jobs.")
        print(f"{OKGREEN}    --doc{ENDC}     runs only the doc tests")
        print()
    
    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1
    
    command = sys.argv[2:]

    if "-j" not in command and "--jobs" not in command:
        command.append("-j")
        cores = os.cpu_count()
        if cores is not None:
            command.append(str(int(cores // 1.5)))

    if verbose:
        print(command)

    command.insert(0, "cargo")
    command.insert(1, "test")

    try:
        if verbose:
            print(f"{HEADER}{command}{ENDC}")
        subprocess.run(
            command,
            check=True,
            text=True,
        )
    except subprocess.CalledProcessError as e:
        error = e.stderr or ""
        if verbose:
            print(e)
            print(error)
        return 1

    return 0

def doc():
    if sys.argv.count("--help") >= 1 or sys.argv.count("-h") >= 1:
        print()
        print(f"{HEADER}usage:{ENDC} build.py doc <flags>")
        print()
        print(f"{HEADER}flags:{ENDC}")
        print(f"{OKGREEN} -v --verbose{ENDC}               enables verbose output")
        print(f"{OKGREEN} -h --help{ENDC}                  prints help message")
        print()
        
    verbose = sys.argv.count("--verbose") >= 1 or sys.argv.count("-v") >= 1

    env = os.environ.copy()
    env["RUSTDOCFLAGS"] = "-D warnings"

    try:
        if verbose:
            print(f"{HEADER}RUSTDOCFLAGS=\"-D warnings\" cargo doc --all-features --no-deps -v{ENDC}")
        subprocess.run(
            ["cargo", "doc", "--all-features", "--no-deps"] + (["-v"] if verbose else []),
            check=True,
            text=True,
            env=env
        )
    except subprocess.CalledProcessError as e:
        error = e.stderr or ""
        if verbose:
            print(e)
            print(error)
        return 1

    return 0

def main():
    if len(sys.argv) < 2:
        help()
        return 2
    elif sys.argv[1] == "--help" or sys.argv[1] == "-h":
        help()
        return 0
    elif sys.argv[1] == "targets":
        return targets(True)
    elif sys.argv[1] == "fmt":
        return fmt()
    elif sys.argv[1] == "clippy":
        return clippy()
    elif sys.argv[1] == "check":
        return check()
    elif sys.argv[1] == "build":
        return build()
    elif sys.argv[1] == "book":
        return book()
    elif sys.argv[1] == "test":
        return test()
    elif sys.argv[1] == "doc":
        return doc()
    else:
        help()
        return 2

if __name__ == "__main__":
    sys.exit(main())