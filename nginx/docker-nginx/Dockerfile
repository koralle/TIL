FROM centos:latest

RUN yum install -y nginx
COPY ./nginx.conf /etc/nginx/nginx.conf

CMD ["nginx", "-g", "daemon off;"]
