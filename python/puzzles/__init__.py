from pathlib import Path

DATA_DIR = Path(__file__).parent.parent.parent / 'data'

INPUT_ONE = DATA_DIR / 'inputs' / '01.txt'
INPUT_TWO = DATA_DIR / 'inputs' / '02.txt'
INPUT_THREE = DATA_DIR / 'inputs' / '03.txt'

INPUTS = {
    1: INPUT_ONE,
    2: INPUT_TWO,
    3: INPUT_THREE,
    4: DATA_DIR / 'inputs' / '04.txt',
}
