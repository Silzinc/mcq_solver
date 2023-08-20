import os
from create_mcq import create_random_mcq
here_path = os.path.dirname(__file__)

from mcq_solver import solve_from_files

grades_path = here_path + r"/grades.txt"
sheets_path = [here_path + r"/sheets/" + f"{k}.txt" for k in range(1, 401)]
grades_separator = ","
number_of_questions = 120
answer_tokens = "abcdef"

mcq = create_random_mcq(400, number_of_questions, answer_tokens)

result, success = solve_from_files(
	sheets_path, 
	answer_tokens,
	grades_path,
	grades_separator,
	number_of_questions,
	)

print("Original MCQ :")
print(mcq)

if not success:
	print("Solving process failed")
	raise Exception(result)

	print("The solver gives :")
	print(result)
	print("Are they equal ?")
	print(mcq == result)
