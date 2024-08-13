FROM ubuntu:latest
MAINTAINER your_name <email_address>

# Оновлюємо стандартні пакети
RUN apt-get update && apt-get upgrade -y
# Встановлюємо пакети, необхідні для виконання програм на C++
#RUN apt-get install -y libboost-all-dev

# Копіюємо виконуваний файл у контейнер
COPY ./ /app/

# Перевіряємо робочу директорію
WORKDIR /app

# Виконуємо виконуваний файл
CMD ["./azs_local_server_rust"]
