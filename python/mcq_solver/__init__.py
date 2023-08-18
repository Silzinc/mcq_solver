from .mcq_solver import *
from os import listdir
from warnings import warn

__doc__ = mcq_solver.__doc__
if hasattr(mcq_solver, "__all__"):
    __all__ = mcq_solver.__all__

def solve(
    sheets,
    _answer_tokens,
    grades,
    starting_beta = 0.1,
    max_beta = 0.5,
    lambda_inv = 1.01,
):
    if len(set(_answer_tokens)) != len(_answer_tokens):
        return "No duplicate answer token is allowed", False
    answer_tokens = list(_answer_tokens)
    return _solve(
        sheets,
        answer_tokens,
        grades,
        starting_beta,
        max_beta,
        lambda_inv,
    )

def solve_from_files(
    sheets_path,
    _answer_tokens,
    grades_path,
    grades_separator,
    number_of_questions,
    starting_beta = 0.1,
    max_beta = 0.5,
    lambda_inv = 1.01,
):
    if type(sheets_path) == str:
        warn("It is not recommanded to give the directory of sheets as an input")
        sheets_path = listdir(sheets_path)
    if len(set(_answer_tokens)) != len(_answer_tokens):
        return "No duplicate answer token is allowed", False
        
    answer_tokens = list(_answer_tokens)
    return _solve_from_files(
        sheets_path,
        answer_tokens,
        grades_path,
        grades_separator,
        number_of_questions,
        starting_beta,
        max_beta,
        lambda_inv,
    )