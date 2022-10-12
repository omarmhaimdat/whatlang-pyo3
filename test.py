from whatlang import detect, detect_script, detect_lang
import timeit

def main():

    result = detect("Ceci est écrit en français")
    script = detect_script("Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!")
    lang = detect_lang("Ceci est écrit en français")
    print(result)
    print(script)
    print(lang)

if __name__ == "__main__":
    main()
    print(timeit.timeit("detect('Ceci est écrit en français')", setup="from whatlang import detect", number=10000))
