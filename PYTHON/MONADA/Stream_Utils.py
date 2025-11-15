# FUNCIONES QUE AYUDA PARA CREAR STREAMS
from Stream_Monad import StreamMonad

def From_Iterable(Iterable):
    return StreamMonad(Iterable)

def From_File_Lines(Filename):
    def Generator():
        with open(Filename, 'r', encoding='utf-8') as F:
            for Line in F:
                yield Line.rstrip('\n')
    return StreamMonad(Generator())
