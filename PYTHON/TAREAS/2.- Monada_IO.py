class IO:
    def __init__(Self, Effect):
        Self.Effect = Effect

    def __call__(Self):
        return Self.Effect()

def Read_File(FileName):
    def Read_File_Effect():
        with open(FileName, 'r') as File:
            return File.read()

    return IO(Read_File_Effect)

def Print_Contents(Contents):
    def Print_Effect():
        print(Contents)

    return IO(Print_Effect)

Contents = Read_File('D:\\CLOUD\\MEGA\\ESCUELAS\\MAESTRIA\\TECNOLOGIAS DE LA PROGRAMACION\\Documento_Monada.txt')()
Print_Contents(Contents)()