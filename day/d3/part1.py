def get_priority(n: int) -> int:
    return n-97 if n>97 else n-39

def main():
    first_department : bool = [False]*52
    file = open('input.txt', 'r')
    Lines = file.readlines()
    sum : int  = 0
    for line in Lines:
        half : int = len(line)//2
        for i in range(0, half):
            ascii : int = get_priority(ord(line[i]))
            if not first_department[ascii]:
                first_department[ascii] = True
        for i in range(half, len(line)):
            ascii = get_priority(ord(line[i]))
            if first_department[ascii]:
                sum += (ascii + 1)
                break
        first_department = [False for _ in first_department]
    print(sum)

if __name__=="__main__":
    main()