import re
def main():
    with open("input.txt", "r") as file:
        string_numbers = {
            "one": 1, 
            "two": 2,
            "three": 3, 
            "four": 4, 
            "five": 5, 
            "six": 6, 
            "seven": 7, 
            "eight": 8, 
            "nine": 9
            }
        total_calibarations = 0
        content = file.read()
        calibarations = content.split('\n')
    
        for cal in calibarations:
            first_number = -1
            last_number = -1 
            while (len(cal)):
                for num in string_numbers.keys():
                    if re.search(f"^{num}", cal):
                        if first_number == -1:
                            first_number = string_numbers[num] * 10
                            last_number = string_numbers[num]
                        last_number = string_numbers[num]
                if cal[0].isnumeric():
                    if first_number == -1:
                            first_number = int(cal[0]) * 10
                            last_number = int(cal[0])
                    last_number = int(cal[0])
                cal = cal[1:]

            total_calibarations += first_number + last_number

        print(total_calibarations)







if __name__ == "__main__":
    main()