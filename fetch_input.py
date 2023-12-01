import requests
import datetime
import os
import dotenv
import sys

def get_today():
    tz_offset = -5.0 # EST
    tzinfo = datetime.timezone(datetime.timedelta(hours = tz_offset))
    today = datetime.datetime.now(tzinfo)

    if today.month != 12:
        sys.exit('It is not December anymore')

    return today.day


def get_session_cookie():
    dotenv.load_dotenv()
    session_cookie = os.getenv('SESSION_COOKIE')

    if session_cookie is None:
        sys.exit('No session cookie found in the environment')

    return session_cookie


def get_input(day_num, session_cookie):
    session = requests.Session()
    url = f'https://adventofcode.com/2023/day/{day_num}/input'
    response = session.get(url, cookies={'session': session_cookie})

    if response.status_code != 200:
        sys.exit('Failed to fetch input data')

    return response.text


def get_file_path(day_num):
    padded_day_num = f'{day_num:02}'
    relative_path = f'inputs/{padded_day_num}.in'
    curr_directory = os.path.dirname(__file__)
    input_filepath = os.path.join(curr_directory, relative_path)

    return input_filepath


def write_to_file(input, input_path):
    try:
        in_file = open(input_path, mode='x')
        in_file.write(input)
    except FileExistsError:
        sys.exit(f'An input file already exists at {input_path}')
    except FileNotFoundError:
        sys.exit(f'A file could not be created at {input_path}.  Are you sure the path is correct?')
    except:
        sys.exit(f'There was a problem creating and writing to the input file at {input_path}')


def main():
    day_num = get_today()
    print(f'===== Advent of Code 2023 Day {day_num} =====')
    print('Fetching input data...')

    session_cookie = get_session_cookie()
    input = get_input(day_num, session_cookie)
    input_path = get_file_path(day_num)
    write_to_file(input, input_path)

    print(f'...input data written to {input_path}')
    print('============= Good luck! =============')


main()
