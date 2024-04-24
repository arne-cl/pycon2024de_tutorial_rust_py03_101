import pytest

from start_from_scratch_arne import (
    check_reg, count_att, say_hello, sum_as_string, travel_avg)

def test_sum_as_string():
    assert sum_as_string(2,3) == '5'

def test_say_hello():
    assert say_hello("joe") == "Hello joe, how are you?"

def test_check_reg_name_in_list():
    assert check_reg('../ex_2/reg_list.txt', 'John') == 'You are registered!'

def test_check_reg_name_not_in_list():
    assert check_reg('../ex_2/reg_list.txt', 'Joe') == "Sorry, you're not on the list!"

def test_check_reg_name_file_missing():
    with pytest.raises(FileNotFoundError):
        check_reg('noexiste.txt', 'John')

def test_count_att():
    assert count_att(["foo", "bar"]) == 2

def test_travel_avg():
    assert travel_avg({"John": 200, "Steve": 750.5}) == 475.25
