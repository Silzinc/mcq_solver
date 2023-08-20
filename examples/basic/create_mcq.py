from random import choice

def create_random_mcq(number_of_sheets, number_of_questions, answer_tokens):
	mcq = [choice(answer_tokens) for _ in range(number_of_questions)]
	sheets = [
	[choice(answer_tokens) for _ in range(number_of_questions)]
	for _ in range(number_of_sheets)
	]
	grades = [sum(a1 == a2 for a1, a2 in zip(mcq, sheet)) for sheet in sheets]
	return mcq, sheets, grades