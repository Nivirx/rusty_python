import rusty_python

def main():
    filename = "test.random"
    hash = rusty_python.blake3_hash(filename)

    print(f"{hash}")

if __name__ == '__main__':
    main()