# app.rb

require 'sinatra'

# ../static ディレクトリをpublic_folderとして設定
set :public_folder, File.expand_path('../static', __dir__)

get '/api/test' do
  'Hello, Sinatra!'
end