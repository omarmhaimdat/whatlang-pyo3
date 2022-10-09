from whatlang import detect
import timeit

def main():
    result = detect("Je parles français et anglais")
    print(result)

if __name__ == "__main__":
    main()