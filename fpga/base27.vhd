library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
entity base27 is
 port(symbol: in unsigned(4 downto 0); valid: out std_logic; t2,t1,t0: out signed(1 downto 0));
end entity;
architecture rtl of base27 is
begin
 process(all)
 variable n: integer;
 begin
 n:=to_integer(symbol); valid<='0'; t2<="00";t1<="00";t0<="00";
 if n<27 then
 valid<='1';t2<=to_signed(n/9-1,2);t1<=to_signed((n/3) mod 3-1,2);t0<=to_signed(n mod 3-1,2);
 end if;
 end process;
end architecture;
