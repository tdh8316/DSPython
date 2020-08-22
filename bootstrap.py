import logging
import platform
import shutil
import sys
from datetime import date
from subprocess import check_output, check_call

# noinspection PyArgumentList
logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[
        logging.FileHandler(
            "build-{}.log".format(date.today().isoformat()), mode="w", encoding="UTF-8"
        ),
        logging.StreamHandler(stream=sys.stdout),
    ],
)

logging.info("Build Damn Small Python...")

if sys.version_info < (3, 4, 0):
    logging.error("Unsupported python version {}".format(sys.version))
    raise RuntimeError("You must need Python 3.4 or higher version.")

logging.debug("OS: {}".format(platform.platform()))
logging.debug("Python: {}".format(platform.python_version()))


class LLVMNotFoundError(Exception):
    ...


class RustNotFoundError(Exception):
    ...


if not (shutil.which("llc") or shutil.which("llvm-config")):
    raise LLVMNotFoundError(
        "Couldn't find llvm. Is LLVM installed on your system and PATH?"
    )
else:
    logging.debug(
        "LLVM: {}".format(check_output(["llvm-config", "--version"]).decode().strip())
    )

if not (shutil.which("cargo") or shutil.which("rustc")):
    raise RustNotFoundError("Couldn't find rust. Is rust installed on your system?")
else:
    logging.debug(
        "RUST: {}".format(check_output(["rustc", "--version"]).decode().strip())
    )

if not shutil.which("pyinstaller"):
    raise ModuleNotFoundError(
        "Couldn't find pyinstaller. Run: python3 -m pip install pyinstaller"
    )
else:
    logging.debug(
        "PYINSTALLER: {}".format(
            check_output(["pyinstaller", "--version"]).decode().strip()
        )
    )

logging.info("1. Packaging...")
check_call("cargo build ...", shell=True)
