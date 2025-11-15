# ESTRUCTURA DEL FRAMEWORK: CONTAR CUANTAS VECES APARECE 'HOLA'
from Stream_Utils import From_File_Lines

if __name__ == "__main__":
    File = "/home/cheo/Monada/DOCUMENTO.txt"
    Stream = From_File_Lines(File)

    # PASO 1: CONTAR CONCURRENCIAS DE 'Hola' LINEA POR LINEA
    Ocurrencias = (
        Stream
        .Map(lambda Line: Line.upper().split().count("HOLA"))
        .Filter(lambda Count: Count > 0)
    )

    # PASO 2: MOSTRAR LISTA DE OCURRENCIAS LINEA POR LINEA
    Lista_Ocurrencias = Ocurrencias.Run()
    print("OCURRENCIAS DE 'HOLA' POR LINEA:")
    print(Lista_Ocurrencias)

    # PASO 3: SUMAR EL TOTAL DE OCURRENCIAS
    Total = sum(Lista_Ocurrencias)
    print(f"TOTAL DE VECES QUE APARECE 'HOLA': {Total}")