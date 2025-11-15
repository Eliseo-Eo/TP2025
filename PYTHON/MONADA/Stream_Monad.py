from functools import reduce

class StreamMonad:
    def __init__(self, Stream):
         # EL STREAM DEBE SER UN ITERABLE O GENERADOR
        self.Stream = Stream

    def __iter__(self):
         # METODO QUE CONVIERTE LA INSTANCIA DE STREAMMONAD EN ITERABLE
        return iter(self.Stream)

    # METODO BIND PARA CHAINING QUE APLICA UNA FUNCION PARA DEVOLVER STREAMMONAD
    def Bind(self, Func):
        def Generator():
            try:
                for Item in self.Stream:
                    Result = Func(Item)
                    if not isinstance(Result, StreamMonad):
                        raise TypeError("La función en Bind debe devolver un StreamMonad")
                    for Inner in Result:
                        yield Inner
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    # FUNCTOR: MAP QUE TRANSFORMA CADA ELEMENTO CON UNA FUNCION PURA
    def Map(self, Func):
        def Generator():
            try:
                for Item in self.Stream:
                    yield Func(Item)
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    # APLICATIVE: AP QUE APLICA FUNCIONES CONTENIDAS ENN UN STREEMMONAD A LOS VALORES EN OTRO STREAMMONAD
    def AP(self, other):
        def Generator():
            try:
                for Func in self.Stream:
                    for Val in other:
                        yield Func(Val)
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    #FILTER: FILTRA ELEMENTOS SEGUN UN PREDICADO
    def Filter(self, predicate):
        def Generator():
            try:
                for Item in self.Stream:
                    if predicate(Item):
                        yield Item
            except Exception as E:
                raise E
        return StreamMonad(Generator())

    # REDUCE: APLICA UNA FUNCION ACUMULADORA SOBRE EL STREAM
    def Reduce(self, Func, Initializer=None):
        try:
            if Initializer is not None:
                return reduce(Func, self.Stream, Initializer)
            else:
                return reduce(Func, self.Stream)
        except Exception as E:
            raise E
        
    # REVOCER: MANEJO FUNCIONAL DE ERRORES
    def Recover(self, handler):
        def Generator():
            try:
                for Item in self.Stream:
                    yield Item
            except Exception as e:
                Recovery = handler(e)
                if not isinstance(Recovery, StreamMonad):
                    raise TypeError("El handler debe devolver un StreamMonad")
                for Rec_Item in Recovery:
                    yield Rec_Item
        return StreamMonad(Generator())

    # RUN: CONSUME EL STREAM Y DEVUELVE UNA LISTA
    def Run(self):
        return list(self.Stream)