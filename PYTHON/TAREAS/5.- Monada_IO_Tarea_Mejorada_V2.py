# Definimos la clase IO que simula una monada para efectos de entrada/salida (E/S)
class IO:
    # El constructor recibe una función (efecto) y la guarda
    def __init__(Self, Effect):
        Self.Effect = Effect

    # Al llamar a la instancia como si fuera una función, se ejecuta el efecto
    def __call__(Self):
        return Self.Effect()

    # Método 'bind' que permite encadenar operaciones (composición monádica)
    def bind(Self, Function):
        # Devuelve una nueva instancia de IO, que cuando se ejecute:
        # - Ejecuta el efecto actual (self())
        # - Pasa el resultado a la función Function
        # - Ejecuta el efecto devuelto por Function
        return IO(lambda: Function(Self())())
        

# Esta función crea un efecto para imprimir un contenido
def Print_Contents(Contents):
    def Print_Effect():
        print(Contents)  # imprime el contenido
    return IO(Print_Effect)  # lo envuelve en una monada IO


# Esta función crea un efecto para leer un archivo
def Read_File(FileName):
    def Read_File_Effect():
        with open(FileName, 'r') as File:  # abre el archivo en modo lectura
            return File.read()  # lee y devuelve su contenido
    return IO(Read_File_Effect)  # lo envuelve en una monada IO


# Esta función cuenta cuántas veces aparece una palabra en un texto dado
def Count_Word(Contents, Word):
    def Count_Effect():
        # Convierte todo a minúsculas, divide en palabras y cuenta las coincidencias
        Count = Contents.lower().split().count(Word.lower())
        # Imprime el resultado
        print(f"LA PALABRA '{Word}' APARECE {Count} VECES.")
    return IO(Count_Effect)  # lo envuelve en una monada IO


# Aquí comienza la ejecución de la cadena monádica

# 1. Crea un efecto que imprime "PROCESANDO ARCHIVO DE TEXTO"
# 2. Luego, cuando termine de imprimir, ejecuta el siguiente efecto: Read File
# 3. Luego, cuando tenga el contenido del archivo:
# 3a. Primero imprime el contenido en mayúsculas
# 3b. Luego cuenta cuántas veces aparece la palabra "Hola"
# 4. Finalmente, imprime un mensaje indicando que se ha terminado
# El último () ejecuta toda la cadena de efectos
Print_Contents("PROCESANDO ARCHIVO DE TEXTO") \
    .bind(lambda _: Read_File("D:\\CLOUD\\MEGA\\ESCUELAS\\MAESTRIA\\TECNOLOGIAS DE LA PROGRAMACION\\Documento_Monada.txt")) \
    .bind(lambda Contents: 
        Print_Contents(Contents.upper())
        .bind(lambda _: Count_Word(Contents, "HOLA"))
    ) \
    .bind(lambda _: Print_Contents("ARCHIVO PROCESADO EXITOSAMENTE"))()