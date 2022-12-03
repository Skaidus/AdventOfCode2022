def get_priority(n: int) -> int:
    return n-97 if n>97 else n-39

def main():
    elf_pair : bool = [False]*104 # 0 to 51 first, 52 to 103 second
    file = open('input.txt', 'r')
    Lines = file.readlines()
    sum : int = 0
    Zn3 : int = 0
    for line in Lines:
        if Zn3 < 2:
            for c in line:
                index : int  = get_priority(ord(c)) + 52 * Zn3
                if not elf_pair[index]:
                    elf_pair[index] = True
        else:
            for c in line:
                ascii : int = get_priority(ord(c))
                if elf_pair[ascii] and elf_pair[ascii+52]:
                    sum += (ascii + 1)
                    break
            elf_pair = [False for _ in elf_pair]
        Zn3 = (Zn3 + 1)%3
    print(sum)

if __name__=="__main__":
    main()