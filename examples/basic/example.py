from create_mcq import create_random_mcq
from mcq_solver import solve

number_of_sheets = 400
number_of_questions = 120
answer_tokens = "abcdef"

mcq, sheets, grades = create_random_mcq(number_of_sheets, number_of_questions, answer_tokens)

result, success = solve(sheets, answer_tokens, grades)

print("Original MCQ :")
print(mcq)

if not success:
	print("Solving process failed")
	raise Exception(result)

print("The solver gives :")
print(result)
print("Are they equal ?")
print(mcq == result)
