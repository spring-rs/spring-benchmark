package io.github.holmofy.spring;

import jakarta.annotation.Resource;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.SingleColumnRowMapper;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@SpringBootApplication
public class App {

    public static void main(String[] args) {
        SpringApplication.run(App.class, args);
    }

    @Resource
    JdbcTemplate jdbcTemplate;

    @GetMapping("/")
    String helloWorld() {
        return "hello world";
    }

    @GetMapping("/mysql")
    String helloMysql() {
        return jdbcTemplate.queryForObject("select version()", SingleColumnRowMapper.newInstance(String.class));
    }
}