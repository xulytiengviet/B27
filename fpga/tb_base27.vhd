library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
entity tb_base27 is end;
architecture test of tb_base27 is
 signal s: unsigned(4 downto 0);signal v: std_logic;signal a,b,c: signed(1 downto 0);
begin
 dut: entity work.base27 port map(s,v,a,b,c);
 process begin
 for i in 0 to 31 loop
 s<=to_unsigned(i,5);wait for 1 ns;
 if i<27 then assert v='1' and 9*(to_integer(a)+1)+3*(to_integer(b)+1)+to_integer(c)+1=i severity failure;
 else assert v='0' severity failure;end if;
 end loop;
 report "32 vectors passed";wait;
 end process;
end architecture;
