from os import remove, chdir, mkdir
from os.path import join, dirname, exists
from shutil import rmtree
from random import choice

def create_random_mcq():
    here_path = dirname(__file__)
    chdir(here_path)
    sheets_path = join(here_path, "sheets")
    grades_path = join(here_path, "grades.txt")

    if exists(sheets_path):
        rmtree(sheets_path)
    mkdir("sheets")
    if exists(grades_path):
        remove(grades_path)

    mcq = [choice("abcd") for _ in range(120)]

    with open(grades_path, "x") as grades_file:
        for k in range(1, 401): # 400 sheets
            grade = 0
            with open(join(sheets_path, f"{k}.txt"), "x") as sheet_file:
                for j in range(120): # 120 questions
                    answer = choice("abcd")
                    grade += answer == mcq[j]
                    sheet_file.write(answer + "\n") # random abcd answers
            grades_file.write(str(grade) + ",\n") # grades separated with a ,
    return mcq