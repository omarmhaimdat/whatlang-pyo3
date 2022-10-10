from whatlang import detect, detect_script
import timeit

def main():

    result = detect("Ceci est écrit en français")
    script = detect_script("Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!")
    print(result)
    print(script)

if __name__ == "__main__":
    main()