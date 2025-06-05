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
   primary key (teacher_id, course_id)
);

/*==============================================================*/
/* Table: Course                                                */
/*==============================================================*/
create table Course
(
   course_id            varchar(256) not null,
   course_name          varchar(256) not null,
   course_property      int not null,
   hours                int not null,
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
   project_id           varchar(256) not null,
   project_name         varchar(256) not null,
   project_src          varchar(256) not null,
   project_type         int not null,
   total_fund           float not null,
   start_year           int not null,
   end_year             int,
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
   paper_name           varchar(256) not null,
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
      references Course (course_id) on delete restrict on update restrict;

alter table PaperPub add constraint FK_PAPERPUB_REFERENCE_TEACHER foreign key (teacher_id)
      references Teacher (teacher_id) on delete restrict on update restrict;

alter table PaperPub add constraint FK_PAPERPUB_REFERENCE_PUBLICAT foreign key (paper_id)
      references Publication (paper_id) on delete restrict on update restrict;

alter table ProjectResp add constraint FK_PROJECTR_REFERENCE_TEACHER foreign key (teacher_id)
      references Teacher (teacher_id) on delete restrict on update restrict;

alter table ProjectResp add constraint FK_PROJECTR_REFERENCE_PROJECT foreign key (project_id)
      references Project (project_id) on delete restrict on update restrict;

alter table User add constraint FK_USER_REFERENCE_TEACHER foreign key (teacher_id)
      references Teacher (teacher_id) on delete restrict on update restrict;

INSERT INTO User (user_name, user_password, user_privilege) VALUES ('admin', '$2b$12$GrGLVGccuHzAMyjcWfywm.08ZsQ8MWuYN/ldlPSbTXscDP.5w58vC', 2);
INSERT INTO User (user_name, user_password) VALUES ('user', '$2b$12$FcJl2hfmBMtljGYT1qJLheHG2ZkHpmNyFupRguVqYnyUw5flmdrxK');
