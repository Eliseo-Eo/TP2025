from Stream_Monad import StreamMonad
from Stream_Utils import From_Iterable, From_File_Lines
import pytest
import io
import os

# ---------- FIXTURES ----------
@pytest.fixture
def simple_stream():
    return From_Iterable([1, 2, 3, 4, 5])

@pytest.fixture
def text_file(tmp_path):
    """CREA UN ARCHIVO TEMPORAL DE PRUEBA"""
    file_path = tmp_path / "Test.txt"
    file_path.write_text("Hola Mundo\nAdios Mundo\nHola Planeta\n")
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
    # ESPERAMOS: f(x)=x+1 → [2,3,4], f(x)=x*2 → [2,4,6]
    assert result == [2, 3, 4, 2, 4, 6]

# ---------- TESTS DE REDUCE ----------
def test_reduce_sumatoria(simple_stream):
    result = simple_stream.Reduce(lambda acc, x: acc + x, 0)
    assert result == 15

# ---------- TESTS DE RECOVER ----------
def test_recover_manejo_errores():
    def faulty():
        raise ValueError("ERROR DE PRUEBA")

    def handler(e):
        return From_Iterable(["RECUPERADO"])

    faulty_stream = StreamMonad((faulty() for _ in range(1)))  # Lanza Error
    result = faulty_stream.Recover(handler).Run()
    assert result == ["RECUPERADO"]

# ---------- TESTS DE RUN ----------
def test_run_retorna_lista(simple_stream):
    result = simple_stream.Run()
    assert isinstance(result, list)

# ---------- TESTS DE FROM_FILE_LINES ----------
def test_from_file_lines(text_file):
    stream = From_File_Lines(text_file)
    lines = stream.Run()
    assert lines == ["Hola Mundo", "Adios Mundo", "Hola Planeta"]

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
    assert total == 2  # “Hola Mundo” y “Hola Planeta”
