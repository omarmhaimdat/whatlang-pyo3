from whatlang import detect
import timeit

def main():

    result = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!")
    print(result)

if __name__ == "__main__":
    main()