with Ada.Containers.Vectors;

with Ada.Text_IO;
use Ada.Text_IO;
with Ada.Integer_Text_IO;
use Ada.Integer_Text_IO;

procedure Day1 is
    
    package Integer_Vectors is new
        Ada.Containers.Vectors
            (Index_Type => Natural,
             Element_Type => Integer);

    use Integer_Vectors;

    F : File_Type;
    File_Name : constant String := "../inputs/day1";
    V : Vector;
    Curr : Integer := 0;
    Last : Integer := 0;
    P_1 : Integer := 0;
    P_2 : Integer := 0;
begin
    Open (F, In_File, File_Name);
    while not End_Of_File (F) loop
        Get (F, Curr);
        if Last /= 0 and then Curr > Last then
            P_1 := P_1 + 1;
        end if;
        V.Append (Curr); 
        Last := Curr;
    end loop;
    Close (F);
    Put ("Part 1: " & Integer'Image (P_1));
    New_Line;

    for I in V.First_Index .. V.Last_Index - 3 loop
        if V (I) < V (I + 3) then
            P_2 := P_2 + 1;
        end if;
    end loop;
    Put ("Part 2: " & Integer'Image (P_2));
end Day1;
