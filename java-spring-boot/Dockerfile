# mvn builder
FROM maven:3.8.5-openjdk-17-slim AS MAVEN_BUILD
WORKDIR /build/
COPY pom.xml /build/
# build all dependencies for cache
RUN mvn dependency:go-offline -B

COPY src /build/src/
RUN mvn install -Dmaven.test.skip=true

# runner container
FROM openjdk:17-jdk-slim

WORKDIR /app

COPY --from=MAVEN_BUILD /build/target/spring-0.0.1-SNAPSHOT.jar /app/app.jar

EXPOSE 8080

ENTRYPOINT ["sh", "-c", "java -jar app.jar"]