def main():
    with open("input.txt", "r") as file:
        content = file.read()
        calibrations = content.split("\n")
        total_calibrations = 0
        
        for cal in calibrations:
            first_number = -1
            second_number = -1
            for ch in cal:
                if ch.isnumeric():
                    if first_number == -1:
                        first_number = int(ch) * 10
                        second_number = int(ch)
                    second_number = int(ch)
            total_calibrations += (first_number + second_number)
        print(total_calibrations)





if __name__ == "__main__":
    main()