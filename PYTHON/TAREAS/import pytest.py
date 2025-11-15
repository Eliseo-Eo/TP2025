import pytest
from Stream_Monad import StreamMonad
from Stream_Utils import From_Iterable, From_File_Lines
import io
import os

# ---------- FIXTURES ----------
@pytest.fixture
def simple_stream():
    return From_Iterable([1, 2, 3, 4, 5])

@pytest.fixture
def text_file(tmp_path):
    """Crea un archivo temporal de prueba"""
    file_path = tmp_path / "test.txt"
    file_path.write_text("Hola mundo\nAdios mundo\nHOLA planeta\n")
    return str(file_path)

# ---------- TESTS DE MAP ----------
def test_map_doblar_valores(simple_stream):
    result = simple_stream.Map(lambda x: x * 2).Run()
    assert result == [2, 4, 6, 8, 10]

# ---------- TESTS DE FILTER ----------
def test_filter_numeros_pares(simple_stream):
    result = simple_stream.Filter(lambda x: x % 2 == 0).Run()
    assert result == [2, 4]

# ---------- TESTS DE BIND ----------
def test_bind_expandir_valores(simple_stream):
    def duplicar(x):
        return From_Iterable([x, x * 10])
    result = simple_stream.Bind(duplicar).Run()
    assert result == [1, 10, 2, 20, 3, 30, 4, 40, 5, 50]

# ---------- TESTS DE AP ----------
def test_aplicative_application():
    funcs = From_Iterable([lambda x: x + 1, lambda x: x * 2])
    valores = From_Iterable([1, 2, 3])
    result = funcs.AP(valores).Run()
    # Esperamos: f(x)=x+1 → [2,3,4], f(x)=x*2 → [2,4,6]
    assert result == [2, 3, 4, 2, 4, 6]

# ---------- TESTS DE REDUCE ----------
def test_reduce_sumatoria(simple_stream):
    result = simple_stream.Reduce(lambda acc, x: acc + x, 0)
    assert result == 15

# ---------- TESTS DE RECOVER ----------
def test_recover_manejo_errores():
    def faulty():
        raise ValueError("Error de prueba")

    def handler(e):
        return From_Iterable(["recuperado"])

    faulty_stream = StreamMonad((faulty() for _ in range(1)))  # lanza error
    result = faulty_stream.Recover(handler).Run()
    assert result == ["recuperado"]

# ---------- TESTS DE RUN ----------
def test_run_retorna_lista(simple_stream):
    result = simple_stream.Run()
    assert isinstance(result, list)

# ---------- TESTS DE FROM_FILE_LINES ----------
def test_from_file_lines(text_file):
    stream = From_File_Lines(text_file)
    lines = stream.Run()
    assert lines == ["Hola mundo", "Adios mundo", "HOLA planeta"]

# ---------- TEST DE FLUJO COMPLETO ----------
def test_pipeline_completo(text_file):
    stream = From_File_Lines(text_file)
    ocurrencias = (
        stream
        .Map(lambda line: line.upper().split().count("HOLA"))
        .Filter(lambda count: count > 0)
    )
    result = ocurrencias.Run()
    total = sum(result)
    assert total == 2  # “Hola mundo” y “HOLA planeta”
