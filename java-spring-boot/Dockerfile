# mvn builder
FROM maven:3.8.5-openjdk-17-slim AS MAVEN_BUILD
COPY pom.xml /build/
WORKDIR /build/
# build all dependencies for cache
RUN mvn dependency:go-offline -B

COPY --from=NODE_BUILD /build/build/ /build/src/main/resources/public
COPY src /build/src/
RUN mvn install -Dmaven.test.skip=true

# runner container
FROM openjdk:17-jdk-alpine

WORKDIR /app

COPY --from=MAVEN_BUILD /build/target/demo-0.0.1-SNAPSHOT.jar /app/app.jar

EXPOSE 8080

ENTRYPOINT ["java -jar app.jar"]