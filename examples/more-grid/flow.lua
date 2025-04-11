job_title = "Fire II Inviscid Flow"
config.dimensions = 2
config.axisymmetric = false

setGasModel('ideal-air-gas-model.lua')

function hot_center(x, y, z)
   if y < 3 then
      T = 2000.0
   else
      T = 500.0
   end
   return FlowState:new{
      p=1000.0, T=500,
      velx=0.0, vely=0.0
   };
end

flowDict = { initial = hot_center }
bcDict = {}

makeFluidBlocks(bcDict, flowDict)
