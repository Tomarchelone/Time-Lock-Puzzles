# Золотухин Денис Денисович, 162, Принципиально последовательное доказательство выполнения работы.

# Инструкция по сборке и запуску

Вся функциональность описана в библиотеке time-lock-puzzles, чтобы её скомпилировать нужно скачать
соответствующую папку и запустить в ней ```cargo build --release```. Для запуска примера нужно
скачать папку example и запустить в ней ```cargo run --release```. Исходный код и там и там лежит в
директории src.

## Описание
В одноранговых сетях важную роль играет доказательство выполнения работы (Proof-of-Work). Оно используется для подтверждения затраты вычислительных ресурсов, которые часто подменяеют затрату времени.
Однако, популярные реализации PoW являются распараллеливаемыми, что, например, увеличивает разрыв между вычислительными возможностями злоумышленников, задумавшими DoS атаку на приложение, и обычными пользователям,
желающими воспользоваться услугами приложения. Сократить подобный зазор способны принципиально нераспараллеливамые реализации PoW-алгоритмов -- Time-lock puzzles -- реализацией которых для случая разделяемого проверочного секрета и посвящен проект.

Писать я собираюсь на Rust. Язык хорош в плане скорости выполнения кода, но при этом остаётся очень безопасным. Также в языке сделан акцент на криптостойкость.
В качестве основы для нераспараллеливаемых загадок будет использоваться первый способ, описанный в этой статье: http://people.csail.mit.edu/rivest/pubs/RSW96.pdf

Ко второй контрольной точке (20-25 марта) я собираюсь реализовать основную функциональность. А именно две программы, одна из которых генерирует "загадку" и ключ к ней, позволяющий быстро проверить правильность решения, и вторая,
которая ищет решения по данной ей загадке.
Дальше я планирую реализовать разделение ключа между несколькими сторонами, чтобы только вместе они смогли верифицировать правильность решения.
Следующим шагом будет проведение экспериментов по измерению времени работы при различных конфигурациях.

Реализована подключаемая библиотека, которую используют и загадыватель и отгадыватель для создания/решения паззла.

Реализована возможность разделения верификации между несколькими сторонами. Каждый узел загадывает свою загадку для клиента, предоставляя стартовое число a, модуль вычета n = p * q, а также количество возведений в квадрат t.
Они знают о существовании друг друга, поэтому, если требуется выполнить работу T, они выдают загадку размера T / k, где k это количество узлов.
Клиент стартует с одной выбранной загадки. Решая очередную загадку, он использует полученный результат в качестве входа в следующую.
Результатом его работы является, во-первых, порядок решения загадок, а так же ответ на каждую загадку с указанным входом.
После решения клиент отправляет решение каждому узлу. Каждый узел проверяет правильность вычисления СВОЕГО паззла на данном входе.
Таким образом, если все узлы верифицирует правильность решения паззла, то это будет значить, что клиент потратил как минимум время T на вычисления. При этом, из-за того, что при решении следующего паззла используется решение предыдущего, распараллеливание решений не представляется возможным.

Написаны интерфейсы для поднятия сервера-ноды и клиента на хосте.
Сервер может принимать несколько типов сообщений по протоколу TCP:
- Запрос на паззл. Сервер сохраняет id решателя, генерирует загадку и отправляет её обратно.
- Запрос на проверку. Сервер получает последовательнось id и ключей, где каждый следующий вход для паззла это результат решения предыдущего:
	key_1 -> [PUZZLE 1] -> key_2 -> [PUZZLE 2] -> ... -> key_n

Он находит в последовательности свой id и эффективно проверяет, что загадка с данным входом решена
верно. Если это так, то он рассылает всем остальным нодам сообщение об этом
- Сообщение о верности решения паззла, выданного сервером id. Сервер исключает id из множества
серверов, от которых ожидается подтверждение. Если все остальные сервера подтвердили верность решения,
то текущий сервер записывает в блокчейн запрос клиента (самого блокчейна в реализации нет).

Клиент может посылать аналогичные сообщения:
- Запрос на паззл. Клиент получает паззл от сервера и запоминает его. Когда все паззлы получены, он
их решает.
- Запрос на проверку. После решения, клиент рассылает решение всем нодам.


