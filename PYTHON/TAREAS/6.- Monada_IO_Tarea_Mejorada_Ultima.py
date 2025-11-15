from functools import reduce

class StreamMonad:
    def __init__(Self, Stream):
        # EL STREAM DEBE SER UN ITERABLE O GENERADOR
        Self.Stream = Stream

    def __iter__(Self):
        # METODO QUE CONVIERTE LA INSTANCIA DE STREAMMONAD EN ITERABLE
        return iter(Self.Stream)

    # METODO BIND PARA CHAINING QUE APLICA UNA FUNCION PARA DEVOLVER STREAMMONAD
    def Bind(Self, Func):
        def Generator():
            try:
                for Item in Self.Stream:
                    Result = Func(Item)
                    if not isinstance(Result, StreamMonad):
                        raise TypeError("LA FUNCION EN BIND DEBE DEVOLVER UN STREAMMONAD")
                    for Inner in Result:
                        yield Inner
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    # FUNCTOR: MAP QUE TRANSFORMA CADA ELEMENTO CON UNA FUNCION PURA
    def Map(Self, Func):
        def Generator():
            try:
                for Item in Self.Stream:
                    yield Func(Item)
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    # APLICATIVE: AP QUE APLICA FUNCIONES CONTENIDAS ENN UN STREEMMONAD A LOS VALORES EN OTRO STREAMMONAD
    def AP(Self, other):
        def Generator():
            try:
                for Func in Self.Stream:
                    for Val in other:
                        yield Func(Val)
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    #FILTER: FILTRA ELEMENTOS SEGUN UN PREDICADO
    def Filter(Self, predicate):
        def Generator():
            try:
                for Item in Self.Stream:
                    if predicate(Item):
                        yield Item
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    # REDUCE: APLICA UNA FUNCION ACUMULADORA SOBRE EL STREAM
    def Reduce(Self, Func, Initializer=None):
        try:
            if Initializer is not None:
                return reduce(Func, Self.Stream, Initializer)
            else:
                return reduce(Func, Self.Stream)
        except Exception as E:
            raise E

    # REVOCER: MANEJO FUNCIONAL DE ERRORES
    def Recover(Self, handler):
        def Generator():
            try:
                for Item in Self.Stream:
                    yield Item
            except Exception as e:
                Recovery = handler(e)
                if not isinstance(Recovery, StreamMonad):
                    raise TypeError("EL HEANDLER DEBE DEVOLVER UN STREAMMONAD")
                for Rec_Item in Recovery:
                    yield Rec_Item
        return StreamMonad(Generator())

    # RUN: CONSUME EL STREAM Y DEVUELVE UNA LISTA
    def Run(Self):
        return list(Self.Stream)


# FUNCIONES QUE AYUDA PARA CREAR STREAMS
def From_Iterable(Iterable):
    return StreamMonad(Iterable)

def From_File_Lines(Filename):
    def Generator():
        with open(Filename, 'r', encoding='utf-8') as F:
            for Line in F:
                yield Line.rstrip('\n')
    return StreamMonad(Generator())


# ESTRUCTURA DEL FRAMEWORK: CONTAR CUANTAS VECES APARECE 'HOLA'
if __name__ == "__main__":
    File = "D:\\CLOUD\\MEGA\\ESCUELAS\\MAESTRIA\\TECNOLOGIAS DE LA PROGRAMACION\\Documento_Monada.txt"
    Stream = From_File_Lines(File)

    # PASO 1: CONTAR CONCURRENCIAS DE 'Hola' LINEA POR LINEA
    Ocurrencias = (
        Stream
        .Map(lambda Line: Line.upper().split().count("HOLA"))  # Cuenta cuántas veces aparece "monada"
        .Filter(lambda Count: Count > 0)                         # Solo conserva líneas donde aparece al menos una vez
    )

    # PASO 2: MOSTRAR LISTA DE OCURRENCIAS LINEA POR LINEA
    Lista_Ocurrencias = Ocurrencias.Run()
    print("OCURRENCIAS DE 'HOLA' POR LINEA:")
    print(Lista_Ocurrencias)

    # PASO 3: SUMAR EL TOTAL DE OCURRENCIAS
    Total = sum(Lista_Ocurrencias)
    print(f"Total de veces que aparece 'monada': {Total}")