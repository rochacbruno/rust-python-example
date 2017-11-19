def count_doubles( unicode source ):
    """Count number of doubles in a (unicode) string
    
    A double is counted for every character where the
    character at the previous index in the string is
    the same character as the current character. Thus
    the string 'aaa' has two doubles.
    """
    cdef Py_ssize_t count
    count = 0
    if not source:
        return count
    char = source[0]
    for next_char in source[1:]:
        if next_char == char:
            count += 1
        char = next_char
    return count
