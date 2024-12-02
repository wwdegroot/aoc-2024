from pathlib import Path

DATA_DIR = Path(__file__).parent.parent.parent / 'data'

INPUT_ONE = DATA_DIR / 'inputs' / '01.txt'
INPUT_TWO = DATA_DIR / 'inputs' / '02.txt'

INPUTS = {
    1: INPUT_ONE,
    2: INPUT_TWO,
}
