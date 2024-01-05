"""
Test whatlang-pyo3 with unittest
"""

from whatlang import detect, detect_script, detect_lang, batch_detect
import time
import unittest

class TestWhatlang(unittest.TestCase):
    
    def test_detect(self):
        result = detect("Ceci est écrit en français")
        self.assertEqual(result.lang, "fra")
        self.assertGreater(result.confidence, 0.1)

    def test_detect_script(self):
        result = detect_script("Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!")
        self.assertEqual(result.name, "Latin")

    def test_detect_lang(self):
        result = detect_lang("Ceci est écrit en français")
        self.assertEqual(result.lang, "fra")

    def test_batch_detect(self):
        result = batch_detect(["Ceci est écrit en français", "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"])
        self.assertEqual(result[0].lang, "fra")
        self.assertGreater(result[0].confidence, 0.1)
        self.assertEqual(result[1].lang, "epo")
        self.assertGreater(result[1].confidence, 0.5)

    def test_performance(self):
        # Create a list of 10000 texts
        n = 10_000
        texts = ["Ceci est écrit en français"] * n
        print("\n--------------------------Batch detect--------------------------")
        start = time.perf_counter()
        batch_detect(texts, n_jobs=-1)
        end = time.perf_counter()
        batch = end - start
        print(f"Batch detect for {n} texts took {batch} seconds")
        print("--------------------------Single detect-------------------------")
        start = time.perf_counter()
        for text in texts:
            detect(text)
        end = time.perf_counter()
        single = end - start
        print(f"Single detect for {n} texts took {single} seconds")
        self.assertGreaterEqual(single, batch)

if __name__ == "__main__":
    unittest.main()
