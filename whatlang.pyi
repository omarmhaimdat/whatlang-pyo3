class Lang:
    def __init__(self, lang: str) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def to_iso(self) -> None: ...

class Info:
    def __init__(self, lang: str, script: str, confidence: float) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def to_iso(self) -> None: ...


class Script:
    '''
    Script of the text

    Attributes:
        script: Script of the text
    '''
    def __init__(self, script: str) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...


def detect(text: str) -> Info: 
    '''
    Detect language and script of the text

    Args:
        text: Text to detect
    
    Returns:
        Info object with detected language, script and confidence

    Raises:
        ValueError: If text is empty
    
    Example:
        >>> detect('This is English text')
        Info(lang='en', script='Latin', confidence=0.96)

        >>> detect('Это русский текст')
        Info(lang='ru', script='Cyrillic', confidence=0.69)
    '''
    ...

def detect_lang(text: str) -> Lang: 
    '''
    Detect language of the text

    Args:
        text: Text to detect
    
    Returns:
        Lang object with detected language
    
    Raises:
        ValueError: If text is empty
    
    Example:
        >>> detect_lang('This is English text')
        Lang(lang='en')
    
        >>> detect_lang('Это русский текст')
        Lang(lang='ru')
    '''
    ...

def detect_script(text: str) -> Script:
    '''
    Detect script of the text

    Args:
        text: Text to detect
    
    Returns:
        Script object with detected script
    
    Raises:
        ValueError: If text is empty

    Example:
        >>> detect_script('This is English text')
        Script(script='Latin')

        >>> detect_script('Это русский текст')
        Script(script='Cyrillic')

        >>> detect_script('これは日本語のテキストです')
        Script(script='Han')
    '''
    ...
