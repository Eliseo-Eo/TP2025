class IO:
    def __init__(self, Effect):
        self.Effect = Effect

    def __call__(self):
        return self.Effect()

    def bind(self, Function):
        return IO(lambda: Function(self())())

def Print_Contents(Contents):
    def Print_Effect():
        print(Contents)
    return IO(Print_Effect)

def Read_File(FileName):
    def Read_File_Effect():
        with open(FileName, 'r') as File:
            return File.read()
    return IO(Read_File_Effect)

Print_Contents("PROCESANDO ARCHIVO DE TEXTO EXTERNO") \
    .bind(lambda _: Read_File("D:\\CLOUD\\MEGA\\ESCUELAS\\MAESTRIA\\TECNOLOGIAS DE LA PROGRAMACION\\Documento_Monada.txt")) \
    .bind(lambda contents: Print_Contents(contents.upper())) \
    .bind(lambda _: Print_Contents("ARCHIVO PROCESADO EXITOSAMENTE"))()