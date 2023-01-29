from whatlang import detect, detect_script, detect_lang, batch_detect
import time

def main():

    result = detect("Ceci est écrit en français")
    script = detect_script("Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!")
    lang = detect_lang("Ceci est écrit en français")
    batch = batch_detect(["Ceci est écrit en français", "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"])
    print(result)
    print(script)
    print(lang)

def compare_batch_with_single_performance():
    # Create a list of 10000 texts
    from whatlang import detect, batch_detect
    import time
    n = 1_000_000
    texts = ["Ceci est écrit en français"] * n
    print("--------------------------Batch detect--------------------------")
    start = time.perf_counter()
    batch_detect(texts, n_jobs=-1)
    end = time.perf_counter()
    print(f"Batch detect for {n} texts took {end - start} seconds")
    print("--------------------------Single detect--------------------------")
    start = time.perf_counter()
    for text in texts:
        detect(text)
    end = time.perf_counter()
    print(f"Single detect for {n} texts took {end - start} seconds")

if __name__ == "__main__":
    compare_batch_with_single_performance()
