/*==============================================================*/
/* DBMS name:      MySQL 5.0                                    */
/* Created on:     2025/5/25 22:15:17                           */
/*==============================================================*/

-- DROP DATABASE IF EXISTS TESTS;
/* -- CREATE DATABASE TESTS;
alter table ClassTeach 
   drop foreign key FK_CLASSTEA_REFERENCE_TEACHER;

alter table ClassTeach 
   drop foreign key FK_CLASSTEA_REFERENCE_COURSE;

alter table PaperPub 
   drop foreign key FK_PAPERPUB_REFERENCE_TEACHER;

alter table PaperPub 
   drop foreign key FK_PAPERPUB_REFERENCE_PUBLICAT;

alter table ProjectResp 
   drop foreign key FK_PROJECTR_REFERENCE_TEACHER;

alter table ProjectResp 
   drop foreign key FK_PROJECTR_REFERENCE_PROJECT;

alter table User 
   drop foreign key FK_USER_REFERENCE_TEACHER;


alter table ClassTeach 
   drop foreign key FK_CLASSTEA_REFERENCE_TEACHER;

alter table ClassTeach 
   drop foreign key FK_CLASSTEA_REFERENCE_COURSE;

drop table if exists ClassTeach;

drop table if exists Course;


alter table PaperPub 
   drop foreign key FK_PAPERPUB_REFERENCE_TEACHER;

alter table PaperPub 
   drop foreign key FK_PAPERPUB_REFERENCE_PUBLICAT;

drop table if exists PaperPub;

drop table if exists Project;


alter table ProjectResp 
   drop foreign key FK_PROJECTR_REFERENCE_TEACHER;

alter table ProjectResp 
   drop foreign key FK_PROJECTR_REFERENCE_PROJECT;

drop table if exists ProjectResp;

drop table if exists Publication;

drop table if exists Teacher;


alter table User 
   drop foreign key FK_USER_REFERENCE_TEACHER;

drop table if exists User; */

/*==============================================================*/
/* Table: ClassTeach                                            */
/*==============================================================*/

DROP DATABASE IF EXISTS DBLAB;
CREATE DATABASE DBLAB;
USE DBLAB;
create table ClassTeach
(
   teacher_id           char(5) not null,
   course_id            varchar(256) not null,
   course_year          int not null,
   course_semester      int not null,
   resp_hour            int not null,
   primary key (teacher_id, course_id, course_year, course_semester)
);

/*==============================================================*/
/* Table: Course                                                */
/*==============================================================*/
create table Course
(
   course_id            varchar(256) not null,
   course_name          varchar(256) not null,
   course_property      int not null,
   hours                int DEFAULT 0 not null,
   primary key (course_id)
);

/*==============================================================*/
/* Table: PaperPub                                              */
/*==============================================================*/
create table PaperPub
(
   teacher_id           char(5) not null,
   paper_id             int not null,
   ranking              int not null,
   comm_auth            boolean not null,
   primary key (teacher_id, paper_id)
);

/*==============================================================*/
/* Table: Project                                               */
/*==============================================================*/
create table Project
(
   project_id           varchar(256) UNIQUE not null,
   project_name         varchar(256) UNIQUE not null,
   project_src          varchar(256) not null,
   project_type         int not null,
   total_fund           float DEFAULT 0.0 not null,
   start_year           int not null,
   end_year             int,
   secret_level         int DEFAULT 1 not null,
   primary key (project_id)
);

/*==============================================================*/
/* Table: ProjectResp                                           */
/*==============================================================*/
create table ProjectResp
(
   teacher_id           char(5) not null,
   project_id           varchar(256) not null,
   ranking              int not null,
   fund                 float not null,
   primary key (teacher_id, project_id)
);

/*==============================================================*/
/* Table: Publication                                           */
/*==============================================================*/
create table Publication
(
   paper_id             int AUTO_INCREMENT not null,
   paper_name           varchar(256) UNIQUE not null,
   paper_src            varchar(256) not null,
   pub_year             date not null,
   paper_type           int not null,
   paper_level          int not null,
   primary key (paper_id)
);

/*==============================================================*/
/* Table: Teacher                                               */
/*==============================================================*/
create table Teacher
(
   teacher_id           char(5) not null,
   teacher_name         varchar(256) not null,
   teacher_sex          int not null,
   teacher_title        int not null,
   primary key (teacher_id)
);

/*==============================================================*/
/* Table: User                                                  */
/*==============================================================*/
create table User
(
   user_id              int AUTO_INCREMENT not null,
   user_name            varchar(256) UNIQUE not null,
   user_password        varchar(256) not null,
   user_privilege       int not null DEFAULT 1,
   teacher_id           char(5),
   primary key (user_id)
);

alter table ClassTeach add constraint FK_CLASSTEA_REFERENCE_TEACHER foreign key (teacher_id)
      references Teacher (teacher_id) on delete restrict on update restrict;

alter table ClassTeach add constraint FK_CLASSTEA_REFERENCE_COURSE foreign key (course_id)
      references Course (course_id) on delete CASCADE on update restrict;

alter table PaperPub add constraint FK_PAPERPUB_REFERENCE_TEACHER foreign key (teacher_id)
      references Teacher (teacher_id) on delete restrict on update restrict;

alter table PaperPub add constraint FK_PAPERPUB_REFERENCE_PUBLICAT foreign key (paper_id)
      references Publication (paper_id) on delete CASCADE on update restrict;

alter table ProjectResp add constraint FK_PROJECTR_REFERENCE_TEACHER foreign key (teacher_id)
      references Teacher (teacher_id) on delete restrict on update restrict;

alter table ProjectResp add constraint FK_PROJECTR_REFERENCE_PROJECT foreign key (project_id)
      references Project (project_id) on delete CASCADE on update restrict;

alter table User add constraint FK_USER_REFERENCE_TEACHER foreign key (teacher_id)
      references Teacher (teacher_id) on delete restrict on update restrict;

DELIMITER //
CREATE PROCEDURE UpdateProjectTotalFund(IN pid VARCHAR(256))
BEGIN
    UPDATE Project
    SET total_fund = (
        SELECT COALESCE(SUM(fund), 0.0)
        FROM ProjectResp
        WHERE project_id = pid
    )
    WHERE project_id = pid;
END //
CREATE TRIGGER after_project_resp_insert
AFTER INSERT ON ProjectResp
FOR EACH ROW
BEGIN
   CALL UpdateProjectTotalFund(NEW.project_id);
END //
CREATE TRIGGER after_project_resp_update
AFTER UPDATE ON ProjectResp
FOR EACH ROW
BEGIN
   CALL UpdateProjectTotalFund(NEW.project_id);
END //
CREATE TRIGGER after_project_resp_delete
AFTER DELETE ON ProjectResp
FOR EACH ROW
BEGIN
   CALL UpdateProjectTotalFund(OLD.project_id);
END //

CREATE PROCEDURE UpdateCourseTotalHours(IN cid VARCHAR(256))
BEGIN
  UPDATE Course
  SET hours = (
    SELECT COALESCE(SUM(resp_hour), 0)
    FROM ClassTeach
    WHERE course_id = cid
  )
  WHERE course_id = cid;
END //
CREATE TRIGGER after_classteach_insert
AFTER INSERT ON ClassTeach
FOR EACH ROW
BEGIN
  CALL UpdateCourseTotalHours(NEW.course_id);
END //
CREATE TRIGGER after_classteach_update
AFTER UPDATE ON ClassTeach
FOR EACH ROW
BEGIN
  CALL UpdateCourseTotalHours(NEW.course_id);
END //
CREATE TRIGGER after_classteach_delete
AFTER DELETE ON ClassTeach
FOR EACH ROW
BEGIN
  CALL UpdateCourseTotalHours(OLD.course_id);
END //
DELIMITER ;

/* use DBLAB;
UPDATE User
SET user_privilege = 2
WHERE user_name = 'admin'; */