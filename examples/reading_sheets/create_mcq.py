from os import remove, chdir, mkdir
from os.path import join, dirname, exists
from shutil import rmtree
from random import choice

def create_random_mcq(number_of_sheets, number_of_questions, answer_tokens):
	here_path = dirname(__file__)
	chdir(here_path)
	sheets_path = join(here_path, "sheets")
	grades_path = join(here_path, "grades.txt")

	if exists(sheets_path):
		rmtree(sheets_path)
		mkdir("sheets")
		if exists(grades_path):
			remove(grades_path)

			mcq = [choice(answer_tokens) for _ in range(number_of_questions)]

			with open(grades_path, "x") as grades_file:
				for k in range(1, number_of_sheets + 1):
					grade = 0
					with open(join(sheets_path, f"{k}.txt"), "x") as sheet_file:
						for j in range(number_of_questions):
							answer = choice(answer_tokens)
							grade += answer == mcq[j]
                    sheet_file.write(answer + "\n") # random answers
            grades_file.write(str(grade) + ",\n") # grades separated with a ,
            return mcq