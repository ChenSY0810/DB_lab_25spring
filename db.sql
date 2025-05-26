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
   course_year          int,
   course_semester      int,
   resp_hour            int,
   primary key (teacher_id, course_id)
);

/*==============================================================*/
/* Table: Course                                                */
/*==============================================================*/
create table Course
(
   course_id            varchar(256) not null,
   course_name          varchar(256),
   course_property      int,
   hours                int,
   primary key (course_id)
);

/*==============================================================*/
/* Table: PaperPub                                              */
/*==============================================================*/
create table PaperPub
(
   teacher_id           char(5) not null,
   paper_id             int not null,
   ranking              int,
   comm_auth            boolean,
   primary key (teacher_id, paper_id)
);

/*==============================================================*/
/* Table: Project                                               */
/*==============================================================*/
create table Project
(
   project_id           varchar(256) not null,
   project_name         varchar(256),
   project_src          varchar(256),
   project_type         int,
   total_fund           float,
   start_year           int,
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
   ranking              int,
   fund                 float,
   primary key (teacher_id, project_id)
);

/*==============================================================*/
/* Table: Publication                                           */
/*==============================================================*/
create table Publication
(
   paper_id             int not null,
   paper_name           varchar(256),
   paper_src            varchar(256),
   pub_year             date,
   paper_type           int,
   paper_level          int,
   primary key (paper_id)
);

/*==============================================================*/
/* Table: Teacher                                               */
/*==============================================================*/
create table Teacher
(
   teacher_id           char(5) not null,
   teacher_name         varchar(256),
   teacher_sex          int,
   teacher_title        int,
   primary key (teacher_id)
);

/*==============================================================*/
/* Table: User                                                  */
/*==============================================================*/
create table User
(
   user_id              int not null,
   user_name            varchar(256),
   user_password        varchar(256),
   user_privilege       int,
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

