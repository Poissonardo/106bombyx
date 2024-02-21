## EPITECH PROJECT, 2023
## Makefile
## File description:
## For 106bombyx
##

NAME    =    106bombyx

SRCS    =    main.py

OBJS    =    $(SRCS)

all:    $(NAME)

$(NAME):        $(OBJS)
	@cp $(SRCS) $(NAME)
	@chmod u+x $(NAME)

clean:
	@rm $(NAME)

fclean: clean
	@rm -f $(NAME)
