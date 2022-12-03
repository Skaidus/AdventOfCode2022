# read file
# declare bool list
# sum = 0
# for each line
# size/2, 

def get_priority(n: int) -> int:
    return n-97 if n>97 else n-39

def main():
    first_department = [False]*52
    file = open('input.txt', 'r')
    Lines = file.readlines()
    sum = 0
    for line in Lines:
        half = len(line)//2
        for i in range(0, half):
            ascii = get_priority(ord(line[i]))
            if not first_department[ascii]:
                first_department[ascii] = True
        for i in range(half, len(line)):
            ascii = get_priority(ord(line[i]))
            if first_department[ascii]:
                sum += (ascii + 1)
                break
        first_department = [0 for _ in first_department]
    print(sum)

if __name__=="__main__":
    main()