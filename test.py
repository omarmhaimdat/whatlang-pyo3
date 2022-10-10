from whatlang import detect, detect_script
import timeit

def main():

    result = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!")
    script = detect_script("Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!")
    print(result)
    print(script)

if __name__ == "__main__":
    main()