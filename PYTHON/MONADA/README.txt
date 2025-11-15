1.- ##### Configuración, el archivo debe estar en: /home/Nombre_De_Tu_Usuario/Monada/ + Nombre_Archivo

2.- ##### En mi caso mi usuario se llama "cheo" por lo tanto mi ruta es: /home/cheo/Monada/DOCUMENTO.txt


3.- ##### Explicación detallada del funcionamiento de la clase StreamMonad

4.- ##### Para ejecutar los test debe tener instalado pytest en el entorno
    Si no lo tienen instalado se instala con el siguiente comando en la terminal sudo apt install python3-pytest
    Despues abrir una terminal en la ruta espacificada en mi caso es: home/cheo/Monada/
    Despues se debe ejectuar el siguinte comando pytest -v

La clase StreamMonad es una estructura avanzada de programación funcional diseñada en Python para manejar flujos de datos —también llamados streams— de una manera eficiente, segura y modular. Su propósito principal es permitir el procesamiento de grandes cantidades de información o datos que llegan de manera continua, sin necesidad de cargarlos todos en memoria al mismo tiempo. En términos generales, una mónada es una abstracción matemática y de programación que encapsula un valor o un conjunto de valores junto con las operaciones que pueden aplicarse sobre ellos, permitiendo componer funciones sin perder el contexto. En este caso, el contexto es un flujo de datos iterable o generado dinámicamente. La StreamMonad combina ideas de las estructuras Functor, Applicative y Monad propias de la programación funcional, proporcionando un conjunto de métodos que permiten aplicar funciones, filtrar, combinar, acumular y manejar errores dentro del flujo de manera controlada.

Inicialización y comportamiento básico:

Al crear una instancia de StreamMonad, se recibe un flujo de datos, que puede provenir de una lista, un generador o incluso de un archivo. Esto le permite trabajar de forma perezosa (lazy evaluation), es decir, solo procesa los datos cuando son realmente necesarios. Además, la clase puede comportarse como un iterable, lo que significa que puede utilizarse en bucles o convertirse en una lista fácilmente, manteniendo su compatibilidad con la estructura estándar de Python.

Transformaciones con el método Map:

El método Map permite aplicar una función pura a cada elemento del flujo. Una función pura es aquella que no altera el estado del programa y siempre devuelve el mismo resultado ante una misma entrada. Por ejemplo, si el flujo contiene líneas de texto, Map podría convertir todas a mayúsculas o contar cuántas palabras tiene cada una. Este método no cambia la cantidad de elementos, sino que transforma su contenido de manera individual.

Encadenamiento con el método Bind:

El método Bind es el núcleo del comportamiento monádico. Su función es permitir el encadenamiento de operaciones, donde cada función aplicada a un elemento del flujo puede devolver otro StreamMonad. Esto resulta útil cuando una transformación genera un nuevo conjunto de datos por cada elemento original. Bind se encarga de “aplanar” todos esos resultados en un solo flujo continuo, manteniendo la estructura funcional y evitando bucles anidados. En pocas palabras, Bind permite conectar operaciones complejas una tras otra, de manera fluida y estructurada.

Aplicaciones entre flujos con el método AP:

El método AP (de Applicative) sirve para aplicar funciones que están contenidas dentro de un flujo a los valores de otro flujo. Es decir, si un StreamMonad contiene varias funciones, y otro contiene datos, AP se encarga de aplicar cada función a cada elemento del otro flujo. Este tipo de operación es más avanzada y permite la interacción entre múltiples flujos de datos, manteniendo siempre el mismo estilo funcional y declarativo.

Filtrado con el método Filter:

El método Filter tiene como propósito seleccionar únicamente los elementos que cumplen con una determinada condición lógica. Esta condición se expresa mediante una función llamada predicado, que devuelve True o False. Por ejemplo, si el flujo contiene números, se podría usar Filter para conservar solo aquellos que sean positivos. Este método es útil para depurar o reducir el conjunto de datos antes de realizar cálculos más complejos.

Acumulación de datos con el método Reduce:

El método Reduce permite aplicar una función acumuladora sobre todos los elementos del flujo, combinándolos en un único resultado final. Por ejemplo, si el flujo contiene una serie de números, Reduce puede utilizarse para obtener su suma total, su promedio o incluso una concatenación. Este método es el paso final en muchos procesos funcionales, donde se requiere transformar una secuencia en un solo valor agregado.

Manejo de errores con el método Recover:

Uno de los aspectos más interesantes de esta clase es el método Recover, que introduce un sistema de manejo de errores dentro del mismo flujo de datos. Si ocurre una excepción durante el procesamiento, Recover permite definir una función de recuperación que actúe en lugar de interrumpir la ejecución. Esta función de recuperación también debe devolver otro StreamMonad, asegurando que el flujo continúe sin romper la cadena de operaciones. Gracias a este enfoque, se pueden manejar errores de lectura, conversión o cálculo sin que el programa falle por completo.

Ejecución del flujo con el método Run:

Después de aplicar todas las transformaciones, filtros o acumulaciones deseadas, el método Run ejecuta finalmente el flujo y devuelve una lista con los resultados finales. Este paso es importante porque durante todo el proceso anterior, las operaciones se mantienen “en espera” (es decir, no se ejecutan de inmediato). Solo cuando se invoca Run, el flujo se evalúa realmente y se obtienen los datos procesados.

Creación de flujos desde diferentes fuentes:

La implementación incluye también funciones auxiliares que facilitan la creación de flujos.

Por ejemplo:

Una función que convierte cualquier lista o iterable en un StreamMonad.
Otra que lee un archivo línea por línea, transformando cada línea en un elemento del flujo.
Esto permite aplicar el modelo monádico directamente sobre datos externos, como archivos de texto, registros o secuencias generadas dinámicamente.

Ejemplo de aplicación práctica:

En el ejemplo final incluido en el código, se utiliza StreamMonad para procesar un archivo de texto y contar cuántas veces aparece la palabra “HOLA”.
El flujo se crea a partir del archivo, luego se transforman las líneas en mayúsculas, se cuenta la cantidad de veces que aparece la palabra, se filtran las líneas donde no aparece, y finalmente se suman todas las ocurrencias. Todo este proceso se realiza sin bucles tradicionales, sin estructuras imperativas y con una lógica clara, encadenada y funcional.

Conclusión

En resumen, la clase StreamMonad representa una implementación elegante y funcional de un modelo de procesamiento de datos basado en la teoría de las mónadas.
Su diseño permite componer operaciones complejas sobre flujos de información de forma clara, segura y eficiente, sin alterar el estado global del programa ni requerir estructuras de control tradicionales. Gracias a sus métodos, se pueden realizar tareas como transformar, filtrar, combinar, reducir y manejar errores dentro de un mismo flujo, manteniendo la pureza funcional y la modularidad del código. Este enfoque es especialmente útil en contextos donde se manejan grandes volúmenes de información, flujos continuos de datos o se busca un estilo de programación declarativo y escalable.


5.- ##### Fuentes Bibliograficas

https://dev-to.translate.goog/hamzzak/mastering-monad-design-patterns-simplify-your-python-code-and-boost-efficiency-kal?_x_tr_sl=en&_x_tr_tl=es&_x_tr_hl=es&_x_tr_pto=tc

https://www-miguelfarrajota-com.translate.goog/2021/06/monads-in-python-with-pymonad/?_x_tr_sl=en&_x_tr_tl=es&_x_tr_hl=es&_x_tr_pto=tc

https://softwaremill.com/functional-containers-summary-functor-vs-applicative-vs-monad/

https://www.w3schools.com/python/default.asp
