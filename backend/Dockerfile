FROM python:3.10.0a7-buster
ENV   PYTHONUNBUFFERED 1
RUN mkdir /app
COPY requirements.txt /app/
RUN pip3 install -r app/requirements.txt
COPY . /app
WORKDIR /app
EXPOSE 8000
# RUN python manage.py migrate
#CMD ["python", "manage.py", "runserver", "0.0.0.0:8000"]
# python manage.py runserver 0.0.0.0:8000
